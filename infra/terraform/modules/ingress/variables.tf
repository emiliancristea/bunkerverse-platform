# Ingress Module Variables

variable "name_prefix" {
  description = "Prefix for naming resources"
  type        = string

  validation {
    condition     = can(regex("^[a-z][a-z0-9-]*$", var.name_prefix))
    error_message = "Name prefix must start with a letter and contain only lowercase letters, numbers, and hyphens."
  }
}

variable "vpc_id" {
  description = "ID of the VPC where load balancer will be created"
  type        = string
}

variable "public_subnet_ids" {
  description = "List of public subnet IDs for the load balancer"
  type        = list(string)

  validation {
    condition     = length(var.public_subnet_ids) >= 2
    error_message = "At least two public subnet IDs must be provided."
  }
}

#################################################
# EKS Cluster Configuration
#################################################

variable "cluster_name" {
  description = "Name of the EKS cluster"
  type        = string
}

variable "cluster_endpoint" {
  description = "EKS cluster endpoint"
  type        = string
}

variable "cluster_certificate_authority_data" {
  description = "Base64 encoded certificate data required to communicate with the cluster"
  type        = string
}

variable "cluster_oidc_issuer_url" {
  description = "The URL on the EKS cluster for the OpenID Connect identity provider"
  type        = string
}

#################################################
# Certificate Configuration
#################################################

variable "certificate_domain" {
  description = "Domain name for SSL certificate"
  type        = string
  default     = ""
}

variable "certificate_subject_alternative_names" {
  description = "Subject alternative names for the SSL certificate"
  type        = list(string)
  default     = []
}

variable "certificate_validation_method" {
  description = "Method to use for certificate validation (DNS or EMAIL)"
  type        = string
  default     = "DNS"

  validation {
    condition     = contains(["DNS", "EMAIL"], var.certificate_validation_method)
    error_message = "Certificate validation method must be either DNS or EMAIL."
  }
}

variable "existing_certificate_arn" {
  description = "ARN of an existing ACM certificate to use instead of creating a new one"
  type        = string
  default     = ""
}

#################################################
# AWS Load Balancer Controller Configuration
#################################################

variable "aws_load_balancer_controller_chart_version" {
  description = "Version of the AWS Load Balancer Controller Helm chart"
  type        = string
  default     = "1.6.2"
}

variable "aws_load_balancer_controller_image_repository" {
  description = "Docker image repository for AWS Load Balancer Controller"
  type        = string
  default     = "602401143452.dkr.ecr.us-west-2.amazonaws.com/amazon/aws-load-balancer-controller"
}

variable "aws_load_balancer_controller_image_tag" {
  description = "Docker image tag for AWS Load Balancer Controller"
  type        = string
  default     = "v2.6.2"
}

variable "controller_replica_count" {
  description = "Number of replicas for the AWS Load Balancer Controller"
  type        = number
  default     = 2

  validation {
    condition     = var.controller_replica_count >= 1 && var.controller_replica_count <= 10
    error_message = "Controller replica count must be between 1 and 10."
  }
}

variable "enable_cert_manager_integration" {
  description = "Enable cert-manager integration for automatic certificate management"
  type        = bool
  default     = false
}

#################################################
# Ingress Class Configuration
#################################################

variable "ingress_class_name" {
  description = "Name of the ingress class"
  type        = string
  default     = "alb"
}

variable "ingress_namespace" {
  description = "Kubernetes namespace for ingress resources"
  type        = string
  default     = "ingress-nginx"
}

variable "create_namespace" {
  description = "Create the ingress namespace if it doesn't exist"
  type        = bool
  default     = true
}

#################################################
# Load Balancer Configuration
#################################################

variable "load_balancer_type" {
  description = "Type of load balancer (application or network)"
  type        = string
  default     = "application"

  validation {
    condition     = contains(["application", "network"], var.load_balancer_type)
    error_message = "Load balancer type must be either application or network."
  }
}

variable "load_balancer_scheme" {
  description = "Load balancer scheme (internet-facing or internal)"
  type        = string
  default     = "internet-facing"

  validation {
    condition     = contains(["internet-facing", "internal"], var.load_balancer_scheme)
    error_message = "Load balancer scheme must be either internet-facing or internal."
  }
}

variable "load_balancer_idle_timeout" {
  description = "Idle timeout for the load balancer in seconds"
  type        = number
  default     = 60

  validation {
    condition     = var.load_balancer_idle_timeout >= 1 && var.load_balancer_idle_timeout <= 4000
    error_message = "Load balancer idle timeout must be between 1 and 4000 seconds."
  }
}

variable "enable_http2" {
  description = "Enable HTTP/2 support on the load balancer"
  type        = bool
  default     = true
}

variable "ssl_policy" {
  description = "SSL policy for HTTPS listeners"
  type        = string
  default     = "ELBSecurityPolicy-TLS-1-2-2017-01"
}

#################################################
# Security Configuration
#################################################

variable "allowed_cidr_blocks" {
  description = "List of CIDR blocks allowed to access the load balancer"
  type        = list(string)
  default     = ["0.0.0.0/0"]
}

variable "additional_security_group_rules" {
  description = "Additional security group rules for the load balancer"
  type = map(object({
    type        = string
    from_port   = number
    to_port     = number
    protocol    = string
    cidr_blocks = optional(list(string))
    description = string
  }))
  default = {}
}

variable "enable_deletion_protection" {
  description = "Enable deletion protection for the load balancer"
  type        = bool
  default     = false
}

#################################################
# WAF Integration
#################################################

variable "enable_waf" {
  description = "Enable AWS WAF integration"
  type        = bool
  default     = false
}

variable "waf_acl_arn" {
  description = "ARN of the AWS WAFv2 WebACL to associate with the load balancer"
  type        = string
  default     = ""
}

#################################################
# Access Logging Configuration
#################################################

variable "enable_access_logs" {
  description = "Enable access logging for the load balancer"
  type        = bool
  default     = true
}

variable "access_logs_s3_bucket" {
  description = "S3 bucket name for access logs. If empty, a bucket will be created."
  type        = string
  default     = ""
}

variable "access_logs_s3_prefix" {
  description = "S3 prefix for access logs"
  type        = string
  default     = "alb-logs"
}

variable "access_logs_retention_days" {
  description = "Number of days to retain access logs"
  type        = number
  default     = 30

  validation {
    condition     = var.access_logs_retention_days >= 1 && var.access_logs_retention_days <= 365
    error_message = "Access logs retention days must be between 1 and 365."
  }
}

#################################################
# Monitoring Configuration
#################################################

variable "enable_monitoring" {
  description = "Enable CloudWatch monitoring for the load balancer"
  type        = bool
  default     = true
}

variable "cloudwatch_alarm_actions" {
  description = "List of ARNs to notify when CloudWatch alarms trigger"
  type        = list(string)
  default     = []
}

variable "enable_cross_zone_load_balancing" {
  description = "Enable cross-zone load balancing"
  type        = bool
  default     = true
}

#################################################
# Default Backend Configuration
#################################################

variable "create_default_backend" {
  description = "Create a default backend service for unmatched requests"
  type        = bool
  default     = true
}

variable "default_backend_image" {
  description = "Docker image for the default backend"
  type        = string
  default     = "registry.k8s.io/defaultbackend-amd64:1.5"
}

#################################################
# Node Selection and Tolerations
#################################################

variable "node_selector" {
  description = "Node selector for AWS Load Balancer Controller pods"
  type        = map(string)
  default     = {}
}

variable "tolerations" {
  description = "Tolerations for AWS Load Balancer Controller pods"
  type = list(object({
    key      = string
    operator = string
    value    = optional(string)
    effect   = string
  }))
  default = []
}

variable "affinity" {
  description = "Affinity rules for AWS Load Balancer Controller pods"
  type        = any
  default     = {}
}

#################################################
# Resource Limits
#################################################

variable "controller_resources" {
  description = "Resource requests and limits for the controller"
  type = object({
    requests = object({
      cpu    = string
      memory = string
    })
    limits = object({
      cpu    = string
      memory = string
    })
  })
  default = {
    requests = {
      cpu    = "100m"
      memory = "200Mi"
    }
    limits = {
      cpu    = "200m"
      memory = "500Mi"
    }
  }
}

#################################################
# High Availability Configuration
#################################################

variable "enable_pod_disruption_budget" {
  description = "Create a pod disruption budget for the controller"
  type        = bool
  default     = true
}

variable "pod_disruption_budget_min_available" {
  description = "Minimum number of pods that must be available during disruption"
  type        = number
  default     = 1
}

#################################################
# Advanced Configuration
#################################################

variable "enable_shield_advanced" {
  description = "Enable AWS Shield Advanced protection"
  type        = bool
  default     = false
}

variable "target_type" {
  description = "Target type for the load balancer (ip or instance)"
  type        = string
  default     = "ip"

  validation {
    condition     = contains(["ip", "instance"], var.target_type)
    error_message = "Target type must be either ip or instance."
  }
}

variable "load_balancer_attributes" {
  description = "Additional load balancer attributes"
  type        = map(string)
  default     = {}
}

variable "target_group_attributes" {
  description = "Default target group attributes"
  type        = map(string)
  default = {
    "deregistration_delay.timeout_seconds" = "300"
    "health_check.enabled"                 = "true"
    "health_check.healthy_threshold"       = "2"
    "health_check.interval"                = "30"
    "health_check.matcher"                 = "200"
    "health_check.path"                    = "/"
    "health_check.port"                    = "traffic-port"
    "health_check.protocol"                = "HTTP"
    "health_check.timeout"                 = "5"
    "health_check.unhealthy_threshold"     = "2"
  }
}

#################################################
# Integration Configuration
#################################################

variable "enable_external_dns_integration" {
  description = "Enable integration with external-dns for automatic DNS record creation"
  type        = bool
  default     = false
}

variable "external_dns_zone_id" {
  description = "Route53 hosted zone ID for external-dns integration"
  type        = string
  default     = ""
}

variable "enable_service_monitor" {
  description = "Enable Prometheus ServiceMonitor for metrics collection"
  type        = bool
  default     = false
}

#################################################
# Cost Optimization
#################################################

variable "enable_cross_zone_load_balancing_cost_optimization" {
  description = "Optimize cross-zone load balancing for cost (may impact availability)"
  type        = bool
  default     = false
}

variable "target_group_health_check_grace_period" {
  description = "Grace period for target group health checks in seconds"
  type        = number
  default     = 300
}

variable "tags" {
  description = "A map of tags to assign to the resources"
  type        = map(string)
  default     = {}
}