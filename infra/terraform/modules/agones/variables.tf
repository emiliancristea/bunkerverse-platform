# Agones Module Variables

#################################################
# General Configuration

variable "cluster_name" {
  description = "Name of the EKS cluster where Agones will be deployed"
  type        = string
}

variable "cluster_endpoint" {
  description = "Endpoint for the EKS cluster"
  type        = string
}

variable "cluster_certificate_authority_data" {
  description = "Base64 encoded certificate data required to communicate with the cluster"
  type        = string
}

variable "agones_version" {
  description = "Version of Agones to deploy"
  type        = string
  default     = "1.36.0"
  
  validation {
    condition     = can(regex("^\\d+\\.\\d+\\.\\d+$", var.agones_version))
    error_message = "Agones version must be in semantic version format (e.g., 1.36.0)."
  }
}

variable "tags" {
  description = "A map of tags to assign to resources"
  type        = map(string)
  default     = {}
}

#################################################
# Agones Controller Configuration

variable "agones_controller" {
  description = "Configuration for the Agones controller"
  type = object({
    replicas = number
    resources = object({
      requests = object({
        cpu    = string
        memory = string
      })
      limits = object({
        cpu    = string
        memory = string
      })
    })
  })
  default = {
    replicas = 1
    resources = {
      requests = {
        cpu    = "100m"
        memory = "128Mi"
      }
      limits = {
        cpu    = "500m"
        memory = "512Mi"
      }
    }
  }
}

variable "agones_ping" {
  description = "Configuration for the Agones ping service"
  type = object({
    replicas = number
  })
  default = {
    replicas = 1
  }
}

variable "agones_allocator" {
  description = "Configuration for the Agones allocator"
  type = object({
    replicas = number
  })
  default = {
    replicas = 1
  }
}

variable "agones_image" {
  description = "Configuration for Agones container images"
  type = object({
    registry = string
  })
  default = {
    registry = "gcr.io/agones-images"
  }
}

variable "log_level" {
  description = "Log level for Agones controller"
  type        = string
  default     = "Info"
  
  validation {
    condition     = contains(["Debug", "Info", "Warn", "Error"], var.log_level)
    error_message = "Log level must be one of: Debug, Info, Warn, Error."
  }
}

#################################################
# GameServer Configuration

variable "gameserver_config" {
  description = "Configuration for GameServer fleet and individual servers"
  type = object({
    create_fleet = bool
    replicas     = number
    image        = string
    
    # Port configuration
    port_range = object({
      min = number
      max = number
    })
    
    # Resource requirements
    resources = object({
      requests = object({
        cpu    = string
        memory = string
      })
      limits = object({
        cpu    = string
        memory = string
      })
    })
    
    # GameServer template specification
    template = object({
      spec = object({
        ports = list(object({
          name          = string
          portPolicy    = string
          containerPort = number
          protocol      = string
        }))
        health = object({
          disabled               = bool
          initialDelaySeconds    = number
          periodSeconds         = number
          failureThreshold      = number
        })
      })
    })
    
    # Auto-scaling configuration
    autoscaling = object({
      enabled       = bool
      min_replicas  = number
      max_replicas  = number
      buffer_size   = string
      sync_interval = number
    })
  })
  
  default = {
    create_fleet = true
    replicas     = 5
    image        = "gcr.io/agones-images/simple-game-server:0.15"
    
    port_range = {
      min = 7000
      max = 8000
    }
    
    resources = {
      requests = {
        cpu    = "100m"
        memory = "64Mi"
      }
      limits = {
        cpu    = "500m"
        memory = "256Mi"
      }
    }
    
    template = {
      spec = {
        ports = [{
          name          = "default"
          portPolicy    = "Dynamic"
          containerPort = 7654
          protocol      = "UDP"
        }]
        health = {
          disabled               = false
          initialDelaySeconds    = 5
          periodSeconds         = 5
          failureThreshold      = 3
        }
      }
    }
    
    autoscaling = {
      enabled       = true
      min_replicas  = 0
      max_replicas  = 20
      buffer_size   = "5"
      sync_interval = 30
    }
  }
  
  validation {
    condition     = var.gameserver_config.port_range.min < var.gameserver_config.port_range.max
    error_message = "Port range minimum must be less than maximum."
  }
  
  validation {
    condition     = var.gameserver_config.port_range.min >= 1024 && var.gameserver_config.port_range.max <= 65535
    error_message = "Port range must be between 1024 and 65535."
  }
  
  validation {
    condition     = var.gameserver_config.autoscaling.min_replicas <= var.gameserver_config.autoscaling.max_replicas
    error_message = "Autoscaling minimum replicas must be less than or equal to maximum replicas."
  }
  
  validation {
    condition     = var.gameserver_config.replicas >= var.gameserver_config.autoscaling.min_replicas
    error_message = "Initial replica count must be greater than or equal to autoscaling minimum."
  }
}

#################################################
# Feature Flags and Optional Components

variable "enable_monitoring" {
  description = "Enable Prometheus monitoring for Agones"
  type        = bool
  default     = true
}

variable "enable_network_policies" {
  description = "Enable Kubernetes NetworkPolicies for enhanced security"
  type        = bool
  default     = true
}

variable "enable_pod_disruption_budgets" {
  description = "Enable PodDisruptionBudgets for high availability"
  type        = bool
  default     = true
}

#################################################
# Advanced Configuration

variable "feature_gates" {
  description = "Feature gates to enable in Agones"
  type        = string
  default     = "PlayerTracking=true,CountsAndLists=true"
}

variable "webhook_cert_dir" {
  description = "Directory where admission webhook certificates are stored"
  type        = string
  default     = "/tmp/k8s-webhook-server/serving-certs"
}

variable "metrics_bind_address" {
  description = "The address the metric endpoint binds to"
  type        = string
  default     = ":8080"
}

variable "health_probe_bind_address" {
  description = "The address the health probe endpoint binds to"
  type        = string
  default     = ":8081"
}

variable "leader_elect" {
  description = "Enable leader election for controller manager"
  type        = bool
  default     = true
}

variable "leader_elect_resource_lock" {
  description = "The type of resource object that is used for locking during leader election"
  type        = string
  default     = "leases"
  
  validation {
    condition     = contains(["leases", "endpoints", "configmaps"], var.leader_elect_resource_lock)
    error_message = "Leader elect resource lock must be one of: leases, endpoints, configmaps."
  }
}

#################################################
# Security Configuration

variable "security_context" {
  description = "Security context for Agones pods"
  type = object({
    run_as_non_root = bool
    run_as_user     = number
    run_as_group    = number
    fs_group        = number
  })
  default = {
    run_as_non_root = true
    run_as_user     = 1000
    run_as_group    = 1000
    fs_group        = 1000
  }
}

variable "pod_security_standards" {
  description = "Pod Security Standards configuration"
  type = object({
    enforce = string
    audit   = string
    warn    = string
  })
  default = {
    enforce = "restricted"
    audit   = "restricted"  
    warn    = "restricted"
  }
  
  validation {
    condition = alltrue([
      contains(["privileged", "baseline", "restricted"], var.pod_security_standards.enforce),
      contains(["privileged", "baseline", "restricted"], var.pod_security_standards.audit),
      contains(["privileged", "baseline", "restricted"], var.pod_security_standards.warn)
    ])
    error_message = "Pod Security Standards levels must be one of: privileged, baseline, restricted."
  }
}

#################################################
# Backup and Disaster Recovery

variable "backup_config" {
  description = "Configuration for Agones state backup"
  type = object({
    enabled          = bool
    schedule         = string
    retention_days   = number
    storage_class    = string
  })
  default = {
    enabled          = false
    schedule         = "0 2 * * *"  # Daily at 2 AM
    retention_days   = 7
    storage_class    = "gp2"
  }
  
  validation {
    condition     = var.backup_config.retention_days > 0 && var.backup_config.retention_days <= 365
    error_message = "Backup retention days must be between 1 and 365."
  }
}

#################################################
# Networking Configuration

variable "service_type" {
  description = "Kubernetes service type for Agones allocator"
  type        = string
  default     = "LoadBalancer"
  
  validation {
    condition     = contains(["ClusterIP", "NodePort", "LoadBalancer"], var.service_type)
    error_message = "Service type must be one of: ClusterIP, NodePort, LoadBalancer."
  }
}

variable "load_balancer_source_ranges" {
  description = "List of CIDR blocks that are allowed to access the LoadBalancer"
  type        = list(string)
  default     = ["0.0.0.0/0"]
}

variable "allocator_tls_config" {
  description = "TLS configuration for the Agones allocator"
  type = object({
    enabled     = bool
    secret_name = string
  })
  default = {
    enabled     = true
    secret_name = "allocator-tls"
  }
}