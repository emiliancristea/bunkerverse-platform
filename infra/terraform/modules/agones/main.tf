# Agones Module - Game Server Management
# Deploys Agones for game server orchestration on Kubernetes
# Includes GameServer Fleet management and auto-scaling capabilities

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.20"
    }
    helm = {
      source  = "hashicorp/helm"
      version = "~> 2.10"
    }
  }
}

# Configure Kubernetes provider using EKS cluster
data "aws_eks_cluster" "cluster" {
  name = var.cluster_name
}

data "aws_eks_cluster_auth" "cluster" {
  name = var.cluster_name
}

provider "kubernetes" {
  host                   = data.aws_eks_cluster.cluster.endpoint
  cluster_ca_certificate = base64decode(data.aws_eks_cluster.cluster.certificate_authority.0.data)
  token                  = data.aws_eks_cluster_auth.cluster.token
}

provider "helm" {
  kubernetes {
    host                   = data.aws_eks_cluster.cluster.endpoint
    cluster_ca_certificate = base64decode(data.aws_eks_cluster.cluster.certificate_authority.0.data)
    token                  = data.aws_eks_cluster_auth.cluster.token
  }
}

#################################################
# Agones System Namespace

resource "kubernetes_namespace" "agones_system" {
  metadata {
    name = "agones-system"
    
    labels = merge(
      var.tags,
      {
        "app.kubernetes.io/name"       = "agones"
        "app.kubernetes.io/version"    = var.agones_version
        "app.kubernetes.io/managed-by" = "terraform"
      }
    )
  }
}

#################################################
# Agones CRDs Installation
# Custom Resource Definitions for GameServers, Fleets, etc.

resource "kubernetes_manifest" "gameserver_crd" {
  manifest = {
    apiVersion = "apiextensions.k8s.io/v1"
    kind       = "CustomResourceDefinition"
    metadata = {
      name = "gameservers.agones.dev"
      labels = {
        "app.kubernetes.io/name"    = "agones"
        "app.kubernetes.io/version" = var.agones_version
      }
    }
    spec = {
      group = "agones.dev"
      scope = "Namespaced"
      names = {
        plural   = "gameservers"
        singular = "gameserver"
        kind     = "GameServer"
        shortNames = ["gs"]
      }
      versions = [{
        name    = "v1"
        served  = true
        storage = true
        schema = {
          openAPIV3Schema = {
            type = "object"
            properties = {
              spec = {
                type = "object"
                properties = {
                  container = {
                    type = "string"
                  }
                  ports = {
                    type = "array"
                    items = {
                      type = "object"
                      properties = {
                        name = {
                          type = "string"
                        }
                        containerPort = {
                          type = "integer"
                        }
                        protocol = {
                          type = "string"
                          enum = ["TCP", "UDP"]
                        }
                        portPolicy = {
                          type = "string"
                          enum = ["Dynamic", "Static", "Passthrough"]
                        }
                      }
                      required = ["containerPort"]
                    }
                  }
                  health = {
                    type = "object"
                    properties = {
                      disabled = {
                        type = "boolean"
                      }
                      initialDelaySeconds = {
                        type = "integer"
                      }
                      periodSeconds = {
                        type = "integer"
                      }
                      failureThreshold = {
                        type = "integer"
                      }
                    }
                  }
                  template = {
                    type = "object"
                  }
                }
                required = ["ports", "template"]
              }
              status = {
                type = "object"
                properties = {
                  state = {
                    type = "string"
                  }
                  address = {
                    type = "string"
                  }
                  ports = {
                    type = "array"
                    items = {
                      type = "object"
                      properties = {
                        name = {
                          type = "string"
                        }
                        port = {
                          type = "integer"
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }]
    }
  }

  depends_on = [kubernetes_namespace.agones_system]
}

resource "kubernetes_manifest" "fleet_crd" {
  manifest = {
    apiVersion = "apiextensions.k8s.io/v1"
    kind       = "CustomResourceDefinition"
    metadata = {
      name = "fleets.agones.dev"
      labels = {
        "app.kubernetes.io/name"    = "agones"
        "app.kubernetes.io/version" = var.agones_version
      }
    }
    spec = {
      group = "agones.dev"
      scope = "Namespaced"
      names = {
        plural   = "fleets"
        singular = "fleet"
        kind     = "Fleet"
      }
      versions = [{
        name    = "v1"
        served  = true
        storage = true
        schema = {
          openAPIV3Schema = {
            type = "object"
            properties = {
              spec = {
                type = "object"
                properties = {
                  replicas = {
                    type = "integer"
                  }
                  strategy = {
                    type = "object"
                    properties = {
                      type = {
                        type = "string"
                        enum = ["Recreate", "RollingUpdate"]
                      }
                      rollingUpdate = {
                        type = "object"
                        properties = {
                          maxUnavailable = {
                            type = "string"
                          }
                          maxSurge = {
                            type = "string"
                          }
                        }
                      }
                    }
                  }
                  template = {
                    type = "object"
                  }
                }
                required = ["replicas", "template"]
              }
            }
          }
        }
      }]
    }
  }

  depends_on = [kubernetes_namespace.agones_system]
}

#################################################
# Agones Controller Installation via Helm

resource "helm_release" "agones" {
  name       = "agones"
  repository = "https://agones.dev/chart/stable"
  chart      = "agones"
  version    = var.agones_version
  namespace  = kubernetes_namespace.agones_system.metadata[0].name

  # Controller configuration
  set {
    name  = "agones.controller.replicas"
    value = var.agones_controller.replicas
  }

  set {
    name  = "agones.controller.resources.requests.cpu"
    value = var.agones_controller.resources.requests.cpu
  }

  set {
    name  = "agones.controller.resources.requests.memory"
    value = var.agones_controller.resources.requests.memory
  }

  set {
    name  = "agones.controller.resources.limits.cpu"
    value = var.agones_controller.resources.limits.cpu
  }

  set {
    name  = "agones.controller.resources.limits.memory"
    value = var.agones_controller.resources.limits.memory
  }

  # Ping service configuration
  set {
    name  = "agones.ping.replicas"
    value = var.agones_ping.replicas
  }

  # Allocator configuration
  set {
    name  = "agones.allocator.replicas"
    value = var.agones_allocator.replicas
  }

  set {
    name  = "agones.allocator.service.serviceType"
    value = "LoadBalancer"
  }

  # Security context
  set {
    name  = "agones.controller.securityContext.runAsNonRoot"
    value = "true"
  }

  set {
    name  = "agones.controller.securityContext.runAsUser"
    value = "1000"
  }

  # Node affinity for game server nodes
  set {
    name  = "agones.controller.nodeSelector.role"
    value = "game-servers"
  }

  # Tolerations for dedicated game server nodes
  set {
    name  = "agones.controller.tolerations[0].key"
    value = "dedicated"
  }

  set {
    name  = "agones.controller.tolerations[0].operator"
    value = "Equal"
  }

  set {
    name  = "agones.controller.tolerations[0].value"
    value = "game-servers"
  }

  set {
    name  = "agones.controller.tolerations[0].effect"
    value = "NoSchedule"
  }

  # Feature gates
  set {
    name  = "agones.featureGates"
    value = "PlayerTracking=true,CountsAndLists=true"
  }

  # Metrics and monitoring
  set {
    name  = "agones.metrics.prometheusEnabled"
    value = "true"
  }

  set {
    name  = "agones.metrics.prometheusServiceDiscovery"
    value = "true"
  }

  depends_on = [
    kubernetes_namespace.agones_system,
    kubernetes_manifest.gameserver_crd,
    kubernetes_manifest.fleet_crd
  ]

  timeout = 600

  values = [
    yamlencode({
      agones = {
        image = {
          registry   = var.agones_image.registry
          tag        = var.agones_version
          pullPolicy = "IfNotPresent"
        }
        
        gameservers = {
          namespaces = ["default", "gameservers"]
          minPort    = var.gameserver_config.port_range.min
          maxPort    = var.gameserver_config.port_range.max
        }

        controller = {
          logLevel = var.log_level
          
          persistentLogs = true
          persistentLogsDir = "/home/agones/logs"
          
          # Health checks
          healthCheck = {
            initialDelaySeconds = 30
            periodSeconds       = 10
            failureThreshold    = 3
          }
        }
      }
    })
  ]
}

#################################################
# Game Server Namespace

resource "kubernetes_namespace" "gameservers" {
  metadata {
    name = "gameservers"
    
    labels = merge(
      var.tags,
      {
        "app.kubernetes.io/name"       = "gameservers"
        "app.kubernetes.io/managed-by" = "terraform"
        "agones.dev/gameserver-ready"  = "true"
      }
    )
  }

  depends_on = [helm_release.agones]
}

#################################################
# GameServer Fleet

resource "kubernetes_manifest" "gameserver_fleet" {
  count = var.gameserver_config.create_fleet ? 1 : 0

  manifest = {
    apiVersion = "agones.dev/v1"
    kind       = "Fleet"
    metadata = {
      name      = "${var.cluster_name}-gameserver-fleet"
      namespace = kubernetes_namespace.gameservers.metadata[0].name
      labels = merge(
        var.tags,
        {
          "app.kubernetes.io/name"       = "gameserver-fleet"
          "app.kubernetes.io/managed-by" = "terraform"
        }
      )
    }
    spec = {
      replicas = var.gameserver_config.replicas
      strategy = {
        type = "RollingUpdate"
        rollingUpdate = {
          maxSurge       = "25%"
          maxUnavailable = "25%"
        }
      }
      template = {
        metadata = {
          labels = {
            version = "v1.0.0"
          }
        }
        spec = merge(
          var.gameserver_config.template.spec,
          {
            health = var.gameserver_config.template.spec.health
            ports  = var.gameserver_config.template.spec.ports
            template = {
              metadata = {
                labels = {
                  app     = "gameserver"
                  version = "v1.0.0"
                }
              }
              spec = {
                containers = [{
                  name  = "gameserver"
                  image = var.gameserver_config.image
                  
                  resources = {
                    requests = {
                      cpu    = var.gameserver_config.resources.requests.cpu
                      memory = var.gameserver_config.resources.requests.memory
                    }
                    limits = {
                      cpu    = var.gameserver_config.resources.limits.cpu
                      memory = var.gameserver_config.resources.limits.memory
                    }
                  }

                  env = [
                    {
                      name = "PORT"
                      value = tostring(var.gameserver_config.template.spec.ports[0].containerPort)
                    }
                  ]
                }]

                # Node selection for dedicated game server nodes
                nodeSelector = {
                  role = "game-servers"
                }

                tolerations = [{
                  key      = "dedicated"
                  operator = "Equal"
                  value    = "game-servers"
                  effect   = "NoSchedule"
                }]
              }
            }
          }
        )
      }
    }
  }

  depends_on = [
    helm_release.agones,
    kubernetes_namespace.gameservers
  ]
}

#################################################
# Fleet Autoscaler

resource "kubernetes_manifest" "gameserver_fleet_autoscaler" {
  count = var.gameserver_config.create_fleet && var.gameserver_config.autoscaling.enabled ? 1 : 0

  manifest = {
    apiVersion = "autoscaling.agones.dev/v1"
    kind       = "FleetAutoscaler"
    metadata = {
      name      = "${var.cluster_name}-gameserver-fleet-autoscaler"
      namespace = kubernetes_namespace.gameservers.metadata[0].name
      labels = merge(
        var.tags,
        {
          "app.kubernetes.io/name"       = "gameserver-fleet-autoscaler"
          "app.kubernetes.io/managed-by" = "terraform"
        }
      )
    }
    spec = {
      fleetName = "${var.cluster_name}-gameserver-fleet"
      policy = {
        type = "Buffer"
        buffer = {
          bufferSize  = var.gameserver_config.autoscaling.buffer_size
          minReplicas = var.gameserver_config.autoscaling.min_replicas
          maxReplicas = var.gameserver_config.autoscaling.max_replicas
        }
      }
      sync = {
        type = "FixedInterval"
        fixedInterval = {
          seconds = var.gameserver_config.autoscaling.sync_interval
        }
      }
    }
  }

  depends_on = [
    kubernetes_manifest.gameserver_fleet,
    helm_release.agones
  ]
}

#################################################
# Monitoring and Observability

# ServiceMonitor for Prometheus scraping
resource "kubernetes_manifest" "agones_controller_servicemonitor" {
  count = var.enable_monitoring ? 1 : 0

  manifest = {
    apiVersion = "monitoring.coreos.com/v1"
    kind       = "ServiceMonitor"
    metadata = {
      name      = "agones-controller"
      namespace = kubernetes_namespace.agones_system.metadata[0].name
      labels = {
        "app.kubernetes.io/name"       = "agones"
        "app.kubernetes.io/component"  = "controller"
        "app.kubernetes.io/managed-by" = "terraform"
      }
    }
    spec = {
      selector = {
        matchLabels = {
          "app.kubernetes.io/name"      = "agones"
          "app.kubernetes.io/component" = "controller"
        }
      }
      endpoints = [{
        port     = "web"
        interval = "30s"
        path     = "/metrics"
      }]
    }
  }

  depends_on = [helm_release.agones]
}

# NetworkPolicy for security
resource "kubernetes_network_policy" "agones_system" {
  count = var.enable_network_policies ? 1 : 0

  metadata {
    name      = "agones-system-network-policy"
    namespace = kubernetes_namespace.agones_system.metadata[0].name
  }

  spec {
    pod_selector {
      match_labels = {
        "app.kubernetes.io/name" = "agones"
      }
    }

    policy_types = ["Ingress", "Egress"]

    ingress {
      from {
        namespace_selector {
          match_labels = {
            name = kubernetes_namespace.gameservers.metadata[0].name
          }
        }
      }
      
      ports {
        protocol = "TCP"
        port     = "8080"
      }
    }

    egress {
      to {
        namespace_selector {
          match_labels = {
            name = "kube-system"
          }
        }
      }
    }

    egress {
      # Allow DNS
      to {}
      ports {
        protocol = "UDP"
        port     = "53"
      }
    }
  }

  depends_on = [helm_release.agones]
}

# PodDisruptionBudget for high availability
resource "kubernetes_pod_disruption_budget_v1" "agones_controller" {
  metadata {
    name      = "agones-controller-pdb"
    namespace = kubernetes_namespace.agones_system.metadata[0].name
  }

  spec {
    min_available = "50%"
    
    selector {
      match_labels = {
        "app.kubernetes.io/name"      = "agones"
        "app.kubernetes.io/component" = "controller"
      }
    }
  }

  depends_on = [helm_release.agones]
}