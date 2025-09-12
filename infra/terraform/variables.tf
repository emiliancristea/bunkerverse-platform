# Bunkerverse Platform - Terraform Variables
# Configuration parameters for infrastructure deployment

#################################################
# Global Configuration
#################################################

variable "project_name" {
  description = "Name of the project"
  type        = string
  default     = "bunkerverse"
  
  validation {
    condition     = can(regex("^[a-z][a-z0-9-]*$", var.project_name))
    error_message = "Project name must start with a letter and contain only lowercase letters, numbers, and hyphens."
  }
}

variable "environment" {
  description = "Environment name (dev, staging, prod)"
  type        = string
  
  validation {
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "Environment must be one of: dev, staging, prod."
  }
}

variable "aws_region" {
  description = "AWS region for resource deployment"
  type        = string
  default     = "us-east-1"
  
  validation {
    condition     = can(regex("^[a-z]{2}-[a-z]+-[0-9]{1}$", var.aws_region))
    error_message = "AWS region must be in the format: us-east-1, eu-west-1, etc."
  }
}

#################################################
# Network Configuration
#################################################

variable "vpc_cidr" {
  description = "CIDR block for VPC"
  type        = string
  default     = "10.0.0.0/16"
  
  validation {
    condition     = can(cidrhost(var.vpc_cidr, 0))
    error_message = "VPC CIDR must be a valid IPv4 CIDR block."
  }
}

variable "enable_nat_gateway" {
  description = "Enable NAT Gateway for private subnets"
  type        = bool
  default     = true
}

variable "single_nat_gateway" {
  description = "Use a single NAT Gateway for all private subnets"
  type        = bool
  default     = false
}

#################################################
# Security Configuration
#################################################

variable "domain_name" {
  description = "Domain name for SSL certificate and ingress"
  type        = string
  default     = ""
}

variable "jwt_secret" {
  description = "JWT secret for authentication services"
  type        = string
  sensitive   = true
  default     = ""
}

variable "enable_encryption_at_rest" {
  description = "Enable encryption at rest for databases and storage"
  type        = bool
  default     = true
}

variable "enable_encryption_in_transit" {
  description = "Enable encryption in transit"
  type        = bool
  default     = true
}

#################################################
# Kubernetes Configuration
#################################################

variable "cluster_version" {
  description = "Kubernetes version for EKS cluster"
  type        = string
  default     = "1.28"
}

variable "node_group_instance_types" {
  description = "Instance types for EKS node groups"
  type        = list(string)
  default     = ["t3.medium", "t3.large"]
}

variable "node_group_capacity_type" {
  description = "Capacity type for EKS node groups (ON_DEMAND, SPOT)"
  type        = string
  default     = "ON_DEMAND"
  
  validation {
    condition     = contains(["ON_DEMAND", "SPOT"], var.node_group_capacity_type)
    error_message = "Capacity type must be either ON_DEMAND or SPOT."
  }
}

variable "node_group_desired_size" {
  description = "Desired number of nodes in the node group"
  type        = number
  default     = 3
  
  validation {
    condition     = var.node_group_desired_size >= 1 && var.node_group_desired_size <= 100
    error_message = "Node group desired size must be between 1 and 100."
  }
}

variable "node_group_max_size" {
  description = "Maximum number of nodes in the node group"
  type        = number
  default     = 10
  
  validation {
    condition     = var.node_group_max_size >= 1 && var.node_group_max_size <= 100
    error_message = "Node group max size must be between 1 and 100."
  }
}

variable "node_group_min_size" {
  description = "Minimum number of nodes in the node group"
  type        = number
  default     = 1
  
  validation {
    condition     = var.node_group_min_size >= 0 && var.node_group_min_size <= 100
    error_message = "Node group min size must be between 0 and 100."
  }
}

#################################################
# Database Configuration
#################################################

variable "rds_instance_class" {
  description = "RDS instance class"
  type        = string
  default     = "db.t3.micro"
}

variable "rds_allocated_storage" {
  description = "Initial allocated storage for RDS instance (GB)"
  type        = number
  default     = 20
  
  validation {
    condition     = var.rds_allocated_storage >= 20 && var.rds_allocated_storage <= 65536
    error_message = "RDS allocated storage must be between 20 and 65536 GB."
  }
}

variable "rds_max_allocated_storage" {
  description = "Maximum allocated storage for RDS instance (GB)"
  type        = number
  default     = 100
}

variable "rds_backup_retention_period" {
  description = "Backup retention period for RDS (days)"
  type        = number
  default     = 7
  
  validation {
    condition     = var.rds_backup_retention_period >= 0 && var.rds_backup_retention_period <= 35
    error_message = "RDS backup retention period must be between 0 and 35 days."
  }
}

variable "redis_node_type" {
  description = "ElastiCache Redis node type"
  type        = string
  default     = "cache.t3.micro"
}

variable "redis_num_cache_clusters" {
  description = "Number of cache clusters in the replication group"
  type        = number
  default     = 1
  
  validation {
    condition     = var.redis_num_cache_clusters >= 1 && var.redis_num_cache_clusters <= 6
    error_message = "Redis cache clusters must be between 1 and 6."
  }
}

variable "opensearch_instance_type" {
  description = "OpenSearch instance type"
  type        = string
  default     = "t3.small.search"
}

variable "opensearch_instance_count" {
  description = "Number of OpenSearch instances"
  type        = number
  default     = 1
  
  validation {
    condition     = var.opensearch_instance_count >= 1 && var.opensearch_instance_count <= 80
    error_message = "OpenSearch instance count must be between 1 and 80."
  }
}

variable "opensearch_volume_size" {
  description = "EBS volume size for OpenSearch (GB)"
  type        = number
  default     = 20
  
  validation {
    condition     = var.opensearch_volume_size >= 10 && var.opensearch_volume_size <= 3584
    error_message = "OpenSearch volume size must be between 10 and 3584 GB."
  }
}

#################################################
# Game Server Configuration
#################################################

variable "agones_version" {
  description = "Agones version for game server management"
  type        = string
  default     = "1.36.0"
}

variable "gameserver_fleet_replicas" {
  description = "Number of game server replicas"
  type        = number
  default     = 1
  
  validation {
    condition     = var.gameserver_fleet_replicas >= 0 && var.gameserver_fleet_replicas <= 100
    error_message = "Game server fleet replicas must be between 0 and 100."
  }
}

#################################################
# Feature Flags
#################################################

variable "enable_monitoring" {
  description = "Enable monitoring and observability stack"
  type        = bool
  default     = true
}

variable "enable_logging" {
  description = "Enable centralized logging"
  type        = bool
  default     = true
}

variable "enable_backup" {
  description = "Enable automated backups"
  type        = bool
  default     = true
}

variable "enable_multi_az" {
  description = "Enable Multi-AZ deployment for databases"
  type        = bool
  default     = false
}

variable "enable_performance_insights" {
  description = "Enable Performance Insights for RDS"
  type        = bool
  default     = false
}

#################################################
# Cost Optimization
#################################################

variable "enable_spot_instances" {
  description = "Enable Spot instances for cost optimization"
  type        = bool
  default     = false
}

variable "enable_scheduled_scaling" {
  description = "Enable scheduled scaling for predictable workloads"
  type        = bool
  default     = false
}

#################################################
# Compliance and Security
#################################################

variable "enable_vpc_flow_logs" {
  description = "Enable VPC Flow Logs for network monitoring"
  type        = bool
  default     = true
}

variable "enable_config_rules" {
  description = "Enable AWS Config rules for compliance"
  type        = bool
  default     = false
}

variable "enable_guardduty" {
  description = "Enable AWS GuardDuty for threat detection"
  type        = bool
  default     = false
}

variable "enable_security_hub" {
  description = "Enable AWS Security Hub for security posture management"
  type        = bool
  default     = false
}

#################################################
# Additional Tags
#################################################

variable "additional_tags" {
  description = "Additional tags to apply to all resources"
  type        = map(string)
  default     = {}
}