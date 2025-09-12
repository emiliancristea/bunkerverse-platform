# Databases Module Variables

variable "name_prefix" {
  description = "Prefix for naming resources"
  type        = string

  validation {
    condition     = can(regex("^[a-z][a-z0-9-]*$", var.name_prefix))
    error_message = "Name prefix must start with a letter and contain only lowercase letters, numbers, and hyphens."
  }
}

variable "vpc_id" {
  description = "ID of the VPC where databases will be created"
  type        = string
}

variable "database_subnet_ids" {
  description = "List of subnet IDs for the database subnet group"
  type        = list(string)

  validation {
    condition     = length(var.database_subnet_ids) >= 2
    error_message = "At least two subnet IDs must be provided for database subnet group."
  }
}

# RDS Configuration
variable "rds_config" {
  description = "RDS MySQL configuration"
  type = object({
    engine         = string
    engine_version = string
    instance_class = string
    
    allocated_storage     = number
    max_allocated_storage = number
    
    database_name = string
    username      = string
    
    backup_retention_period = number
    backup_window          = string
    maintenance_window     = string
    
    deletion_protection = bool
    skip_final_snapshot = bool
    
    performance_insights_enabled = bool
    monitoring_interval         = number
    
    enabled_cloudwatch_logs_exports = list(string)
  })

  validation {
    condition = contains([
      "mysql"
    ], var.rds_config.engine)
    error_message = "RDS engine must be mysql."
  }

  validation {
    condition = var.rds_config.allocated_storage >= 20 && var.rds_config.allocated_storage <= 65536
    error_message = "RDS allocated storage must be between 20 and 65536 GB."
  }

  validation {
    condition = var.rds_config.backup_retention_period >= 0 && var.rds_config.backup_retention_period <= 35
    error_message = "Backup retention period must be between 0 and 35 days."
  }

  validation {
    condition = var.rds_config.monitoring_interval == 0 || contains([1, 5, 10, 15, 30, 60], var.rds_config.monitoring_interval)
    error_message = "Monitoring interval must be 0 or one of: 1, 5, 10, 15, 30, 60."
  }
}

# ElastiCache Redis Configuration
variable "redis_config" {
  description = "ElastiCache Redis configuration"
  type = object({
    node_type               = string
    num_cache_clusters      = number
    parameter_group_name    = string
    port                   = number
    
    snapshot_retention_limit = number
    snapshot_window         = string
    maintenance_window      = string
    
    at_rest_encryption_enabled = bool
    transit_encryption_enabled = bool
    auth_token_enabled         = bool
  })

  validation {
    condition = var.redis_config.num_cache_clusters >= 1 && var.redis_config.num_cache_clusters <= 6
    error_message = "Number of cache clusters must be between 1 and 6."
  }

  validation {
    condition = var.redis_config.port > 0 && var.redis_config.port <= 65535
    error_message = "Redis port must be between 1 and 65535."
  }

  validation {
    condition = var.redis_config.snapshot_retention_limit >= 0 && var.redis_config.snapshot_retention_limit <= 35
    error_message = "Snapshot retention limit must be between 0 and 35 days."
  }
}

# OpenSearch Configuration
variable "opensearch_config" {
  description = "OpenSearch configuration"
  type = object({
    engine_version = string
    instance_type  = string
    instance_count = number
    
    dedicated_master_enabled = bool
    master_instance_type     = string
    master_instance_count    = number
    
    ebs_enabled = bool
    volume_type = string
    volume_size = number
    
    encrypt_at_rest = object({
      enabled = bool
    })
    
    node_to_node_encryption = object({
      enabled = bool
    })
    
    domain_endpoint_options = object({
      enforce_https       = bool
      tls_security_policy = string
    })
  })

  validation {
    condition = var.opensearch_config.instance_count >= 1 && var.opensearch_config.instance_count <= 80
    error_message = "OpenSearch instance count must be between 1 and 80."
  }

  validation {
    condition = var.opensearch_config.volume_size >= 10 && var.opensearch_config.volume_size <= 3584
    error_message = "OpenSearch volume size must be between 10 and 3584 GB."
  }

  validation {
    condition = contains(["gp2", "gp3", "io1", "io2"], var.opensearch_config.volume_type)
    error_message = "OpenSearch volume type must be one of: gp2, gp3, io1, io2."
  }

  validation {
    condition = !var.opensearch_config.dedicated_master_enabled || (var.opensearch_config.master_instance_count >= 3 && var.opensearch_config.master_instance_count % 2 == 1)
    error_message = "Master instance count must be an odd number >= 3 when dedicated master is enabled."
  }
}

# Security Configuration
variable "enable_multi_az" {
  description = "Enable Multi-AZ deployment for databases"
  type        = bool
  default     = false
}

variable "enable_encryption_at_rest" {
  description = "Enable encryption at rest for all databases"
  type        = bool
  default     = true
}

variable "enable_encryption_in_transit" {
  description = "Enable encryption in transit for all databases"
  type        = bool
  default     = true
}

variable "kms_key_id" {
  description = "KMS key ID for database encryption. If not provided, a key will be created."
  type        = string
  default     = null
}

# Monitoring Configuration
variable "enable_performance_insights" {
  description = "Enable Performance Insights for RDS"
  type        = bool
  default     = false
}

variable "enable_enhanced_monitoring" {
  description = "Enable enhanced monitoring for RDS"
  type        = bool
  default     = false
}

variable "monitoring_interval" {
  description = "The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance"
  type        = number
  default     = 0

  validation {
    condition = var.monitoring_interval == 0 || contains([1, 5, 10, 15, 30, 60], var.monitoring_interval)
    error_message = "Monitoring interval must be 0 or one of: 1, 5, 10, 15, 30, 60."
  }
}

variable "cloudwatch_alarm_actions" {
  description = "List of ARNs to notify when CloudWatch alarms trigger"
  type        = list(string)
  default     = []
}

# Backup Configuration
variable "enable_automated_backups" {
  description = "Enable automated backups for databases"
  type        = bool
  default     = true
}

variable "backup_retention_period" {
  description = "Number of days to retain automated backups"
  type        = number
  default     = 7

  validation {
    condition     = var.backup_retention_period >= 0 && var.backup_retention_period <= 35
    error_message = "Backup retention period must be between 0 and 35 days."
  }
}

variable "backup_window" {
  description = "Preferred backup window"
  type        = string
  default     = "03:00-04:00"
}

variable "maintenance_window" {
  description = "Preferred maintenance window"
  type        = string
  default     = "sun:04:00-sun:05:00"
}

# Logging Configuration
variable "enable_database_logging" {
  description = "Enable database logging to CloudWatch"
  type        = bool
  default     = true
}

variable "log_retention_days" {
  description = "Number of days to retain CloudWatch logs"
  type        = number
  default     = 30

  validation {
    condition = contains([
      1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1827, 3653
    ], var.log_retention_days)
    error_message = "Log retention period must be one of the allowed values."
  }
}

variable "enable_slow_query_log" {
  description = "Enable slow query logging for MySQL"
  type        = bool
  default     = true
}

# Network Configuration
variable "allowed_cidr_blocks" {
  description = "List of CIDR blocks allowed to access databases"
  type        = list(string)
  default     = []
}

variable "allowed_security_group_ids" {
  description = "List of security group IDs allowed to access databases"
  type        = list(string)
  default     = []
}

# High Availability Configuration
variable "enable_read_replica" {
  description = "Enable read replica for RDS"
  type        = bool
  default     = false
}

variable "read_replica_config" {
  description = "Configuration for RDS read replica"
  type = object({
    instance_class = optional(string)
    count         = optional(number, 1)
  })
  default = {
    instance_class = null
    count         = 1
  }
}

# Cost Optimization
variable "enable_deletion_protection" {
  description = "Enable deletion protection for production databases"
  type        = bool
  default     = true
}

variable "skip_final_snapshot" {
  description = "Skip final snapshot when deleting databases (not recommended for production)"
  type        = bool
  default     = false
}

variable "preferred_instance_types" {
  description = "Preferred instance types for cost optimization"
  type = object({
    rds_instance_types       = optional(list(string), ["db.t3.micro", "db.t3.small", "db.r5.large"])
    redis_node_types        = optional(list(string), ["cache.t3.micro", "cache.r5.large"])
    opensearch_instance_types = optional(list(string), ["t3.small.search", "r5.large.search"])
  })
  default = {}
}

# Advanced Configuration
variable "custom_parameter_groups" {
  description = "Custom parameter group configurations"
  type = object({
    rds_parameters = optional(map(string), {})
    redis_parameters = optional(map(string), {})
  })
  default = {}
}

variable "enable_cross_region_backup" {
  description = "Enable cross-region backup replication"
  type        = bool
  default     = false
}

variable "cross_region_backup_config" {
  description = "Cross-region backup configuration"
  type = object({
    destination_region = optional(string)
    kms_key_id        = optional(string)
  })
  default = {}
}

variable "tags" {
  description = "A map of tags to assign to the resources"
  type        = map(string)
  default     = {}
}