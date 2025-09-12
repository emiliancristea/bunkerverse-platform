# Agones Module Outputs

#################################################
# Namespace Information

output "agones_namespace" {
  description = "Name of the Agones system namespace"
  value       = kubernetes_namespace.agones_system.metadata[0].name
}

output "agones_namespace_uid" {
  description = "UID of the Agones system namespace"
  value       = kubernetes_namespace.agones_system.metadata[0].uid
}

output "gameservers_namespace" {
  description = "Name of the GameServers namespace"
  value       = kubernetes_namespace.gameservers.metadata[0].name
}

output "gameservers_namespace_uid" {
  description = "UID of the GameServers namespace"
  value       = kubernetes_namespace.gameservers.metadata[0].uid
}

#################################################
# Agones Helm Release Information

output "agones_chart_version" {
  description = "Version of the deployed Agones Helm chart"
  value       = helm_release.agones.version
}

output "agones_chart_status" {
  description = "Status of the Agones Helm release"
  value       = helm_release.agones.status
}

output "agones_release_name" {
  description = "Name of the Agones Helm release"
  value       = helm_release.agones.name
}

output "agones_chart_name" {
  description = "Name of the Agones Helm chart"
  value       = helm_release.agones.chart
}

#################################################
# GameServer Fleet Information

output "gameserver_fleet_name" {
  description = "Name of the GameServer fleet (if created)"
  value       = var.gameserver_config.create_fleet ? "${var.cluster_name}-gameserver-fleet" : null
}

output "gameserver_fleet_namespace" {
  description = "Namespace of the GameServer fleet (if created)"
  value       = var.gameserver_config.create_fleet ? kubernetes_namespace.gameservers.metadata[0].name : null
}

output "gameserver_fleet_replicas" {
  description = "Number of replicas configured for the GameServer fleet"
  value       = var.gameserver_config.create_fleet ? var.gameserver_config.replicas : null
}

output "gameserver_fleet_autoscaler_name" {
  description = "Name of the Fleet autoscaler (if enabled)"
  value       = var.gameserver_config.create_fleet && var.gameserver_config.autoscaling.enabled ? "${var.cluster_name}-gameserver-fleet-autoscaler" : null
}

#################################################
# Configuration Information

output "agones_version" {
  description = "Version of Agones that was deployed"
  value       = var.agones_version
}

output "gameserver_port_range" {
  description = "Port range configured for GameServers"
  value = {
    min = var.gameserver_config.port_range.min
    max = var.gameserver_config.port_range.max
  }
}

output "gameserver_image" {
  description = "Container image configured for GameServers"
  value       = var.gameserver_config.image
}

#################################################
# Controller Configuration

output "agones_controller_replicas" {
  description = "Number of replicas for the Agones controller"
  value       = var.agones_controller.replicas
}

output "agones_controller_resources" {
  description = "Resource configuration for the Agones controller"
  value       = var.agones_controller.resources
}

output "agones_ping_replicas" {
  description = "Number of replicas for the Agones ping service"
  value       = var.agones_ping.replicas
}

output "agones_allocator_replicas" {
  description = "Number of replicas for the Agones allocator"
  value       = var.agones_allocator.replicas
}

#################################################
# Feature Flags and Configuration

output "feature_gates" {
  description = "Feature gates enabled for Agones"
  value       = var.feature_gates
}

output "log_level" {
  description = "Log level configured for Agones"
  value       = var.log_level
}

output "monitoring_enabled" {
  description = "Whether monitoring is enabled for Agones"
  value       = var.enable_monitoring
}

output "network_policies_enabled" {
  description = "Whether NetworkPolicies are enabled"
  value       = var.enable_network_policies
}

output "pod_disruption_budgets_enabled" {
  description = "Whether PodDisruptionBudgets are enabled"
  value       = var.enable_pod_disruption_budgets
}

#################################################
# Security Configuration

output "security_context" {
  description = "Security context configuration for Agones pods"
  value       = var.security_context
  sensitive   = false
}

output "pod_security_standards" {
  description = "Pod Security Standards configuration"
  value       = var.pod_security_standards
}

#################################################
# Autoscaling Configuration

output "autoscaling_config" {
  description = "Autoscaling configuration for GameServer fleets"
  value = var.gameserver_config.create_fleet && var.gameserver_config.autoscaling.enabled ? {
    enabled       = var.gameserver_config.autoscaling.enabled
    min_replicas  = var.gameserver_config.autoscaling.min_replicas
    max_replicas  = var.gameserver_config.autoscaling.max_replicas
    buffer_size   = var.gameserver_config.autoscaling.buffer_size
    sync_interval = var.gameserver_config.autoscaling.sync_interval
  } : null
}

#################################################
# Networking Configuration

output "service_type" {
  description = "Service type configured for Agones allocator"
  value       = var.service_type
}

output "load_balancer_source_ranges" {
  description = "Source ranges allowed for LoadBalancer access"
  value       = var.load_balancer_source_ranges
}

output "allocator_tls_config" {
  description = "TLS configuration for the Agones allocator"
  value       = var.allocator_tls_config
  sensitive   = true
}

#################################################
# Backup Configuration

output "backup_config" {
  description = "Backup configuration for Agones"
  value       = var.backup_config
}

#################################################
# Custom Resource Definitions

output "gameserver_crd_name" {
  description = "Name of the GameServer CustomResourceDefinition"
  value       = "gameservers.agones.dev"
}

output "fleet_crd_name" {
  description = "Name of the Fleet CustomResourceDefinition"
  value       = "fleets.agones.dev"
}

#################################################
# Labels and Tags

output "resource_labels" {
  description = "Labels applied to Agones resources"
  value = {
    "app.kubernetes.io/name"       = "agones"
    "app.kubernetes.io/version"    = var.agones_version
    "app.kubernetes.io/managed-by" = "terraform"
  }
}

output "applied_tags" {
  description = "Tags applied to AWS resources"
  value       = var.tags
}

#################################################
# Health Check Configuration

output "gameserver_health_check" {
  description = "Health check configuration for GameServers"
  value       = var.gameserver_config.template.spec.health
}

#################################################
# Resource Requirements

output "gameserver_resources" {
  description = "Resource requirements for GameServers"
  value       = var.gameserver_config.resources
}

#################################################
# Port Configuration

output "gameserver_ports" {
  description = "Port configuration for GameServers"
  value       = var.gameserver_config.template.spec.ports
}

#################################################
# Cluster Information

output "target_cluster_name" {
  description = "Name of the EKS cluster where Agones is deployed"
  value       = var.cluster_name
}

output "target_cluster_endpoint" {
  description = "Endpoint of the EKS cluster (sensitive)"
  value       = var.cluster_endpoint
  sensitive   = true
}