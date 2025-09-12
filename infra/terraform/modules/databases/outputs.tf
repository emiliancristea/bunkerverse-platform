# Databases Module Outputs

#################################################
# RDS MySQL Outputs
#################################################

output "rds_instance_id" {
  description = "The RDS instance ID"
  value       = aws_db_instance.main.id
}

output "rds_instance_arn" {
  description = "The ARN of the RDS instance"
  value       = aws_db_instance.main.arn
}

output "rds_instance_endpoint" {
  description = "The RDS instance endpoint"
  value       = aws_db_instance.main.endpoint
}

output "rds_instance_hosted_zone_id" {
  description = "The canonical hosted zone ID of the DB instance"
  value       = aws_db_instance.main.hosted_zone_id
}

output "rds_instance_port" {
  description = "The RDS instance port"
  value       = aws_db_instance.main.port
}

output "rds_instance_status" {
  description = "The RDS instance status"
  value       = aws_db_instance.main.status
}

output "rds_instance_address" {
  description = "The hostname of the RDS instance"
  value       = aws_db_instance.main.address
}

output "rds_database_name" {
  description = "The name of the database"
  value       = aws_db_instance.main.db_name
}

output "rds_username" {
  description = "The master username for the database"
  value       = aws_db_instance.main.username
  sensitive   = true
}

output "rds_password" {
  description = "The master password for the database"
  value       = random_password.rds_password.result
  sensitive   = true
}

output "rds_security_group_id" {
  description = "The ID of the RDS security group"
  value       = aws_security_group.rds.id
}

output "rds_subnet_group_name" {
  description = "The name of the DB subnet group"
  value       = aws_db_subnet_group.main.name
}

output "rds_parameter_group_name" {
  description = "The name of the DB parameter group"
  value       = aws_db_parameter_group.main.name
}

#################################################
# ElastiCache Redis Outputs
#################################################

output "redis_replication_group_id" {
  description = "The ID of the ElastiCache replication group"
  value       = aws_elasticache_replication_group.redis.replication_group_id
}

output "redis_replication_group_arn" {
  description = "The ARN of the ElastiCache replication group"
  value       = aws_elasticache_replication_group.redis.arn
}

output "redis_primary_endpoint_address" {
  description = "The address of the endpoint for the primary node in the replication group"
  value       = aws_elasticache_replication_group.redis.primary_endpoint_address
}

output "redis_reader_endpoint_address" {
  description = "The address of the endpoint for the reader node in the replication group"
  value       = aws_elasticache_replication_group.redis.reader_endpoint_address
}

output "redis_configuration_endpoint_address" {
  description = "The configuration endpoint address to allow host discovery"
  value       = aws_elasticache_replication_group.redis.configuration_endpoint_address
}

output "redis_port" {
  description = "The port number on which each of the cache nodes will accept connections"
  value       = aws_elasticache_replication_group.redis.port
}

output "redis_auth_token" {
  description = "The Redis AUTH token"
  value       = var.redis_config.auth_token_enabled ? random_password.redis_auth_token.result : null
  sensitive   = true
}

output "redis_security_group_id" {
  description = "The ID of the Redis security group"
  value       = aws_security_group.redis.id
}

output "redis_subnet_group_name" {
  description = "The name of the cache subnet group"
  value       = aws_elasticache_subnet_group.main.name
}

output "redis_parameter_group_name" {
  description = "The name of the cache parameter group"
  value       = aws_elasticache_parameter_group.redis.name
}

#################################################
# OpenSearch Outputs
#################################################

output "opensearch_domain_arn" {
  description = "The ARN of the OpenSearch domain"
  value       = aws_opensearch_domain.main.arn
}

output "opensearch_domain_id" {
  description = "The unique identifier for the OpenSearch domain"
  value       = aws_opensearch_domain.main.domain_id
}

output "opensearch_domain_name" {
  description = "The name of the OpenSearch domain"
  value       = aws_opensearch_domain.main.domain_name
}

output "opensearch_endpoint" {
  description = "Domain-specific endpoint used to submit index, search, and data upload requests"
  value       = aws_opensearch_domain.main.endpoint
}

output "opensearch_kibana_endpoint" {
  description = "Domain-specific endpoint for Kibana without https scheme"
  value       = aws_opensearch_domain.main.kibana_endpoint
}

output "opensearch_dashboard_endpoint" {
  description = "Domain-specific endpoint for Dashboard without https scheme"
  value       = aws_opensearch_domain.main.dashboard_endpoint
}

output "opensearch_vpc_options" {
  description = "VPC options for the OpenSearch domain"
  value       = aws_opensearch_domain.main.vpc_options
}

output "opensearch_master_username" {
  description = "The master username for OpenSearch"
  value       = "admin"
  sensitive   = true
}

output "opensearch_master_password" {
  description = "The master password for OpenSearch"
  value       = random_password.opensearch_master_password.result
  sensitive   = true
}

output "opensearch_security_group_id" {
  description = "The ID of the OpenSearch security group"
  value       = aws_security_group.opensearch.id
}

#################################################
# Encryption and Security Outputs
#################################################

output "kms_key_id" {
  description = "The globally unique identifier for the KMS key used for database encryption"
  value       = aws_kms_key.databases.key_id
}

output "kms_key_arn" {
  description = "The Amazon Resource Name (ARN) of the KMS key used for database encryption"
  value       = aws_kms_key.databases.arn
}

output "kms_alias_name" {
  description = "The display name of the KMS key alias"
  value       = aws_kms_alias.databases.name
}

#################################################
# Monitoring Outputs
#################################################

output "cloudwatch_log_groups" {
  description = "CloudWatch log groups created for databases"
  value = {
    redis_slow_log           = aws_cloudwatch_log_group.redis_slow.name
    opensearch_index_slow    = aws_cloudwatch_log_group.opensearch_index_slow.name
    opensearch_search_slow   = aws_cloudwatch_log_group.opensearch_search_slow.name
    opensearch_application   = aws_cloudwatch_log_group.opensearch_application.name
  }
}

output "cloudwatch_alarms" {
  description = "CloudWatch alarms created for database monitoring"
  value = {
    rds_cpu_utilization      = aws_cloudwatch_metric_alarm.rds_cpu_utilization.alarm_name
    rds_database_connections = aws_cloudwatch_metric_alarm.rds_database_connections.alarm_name
    redis_cpu_utilization    = aws_cloudwatch_metric_alarm.redis_cpu_utilization.alarm_name
    opensearch_cluster_status = aws_cloudwatch_metric_alarm.opensearch_cluster_status.alarm_name
  }
}

#################################################
# Connection Information
#################################################

output "database_connections" {
  description = "Database connection information"
  value = {
    mysql = {
      host     = aws_db_instance.main.address
      port     = aws_db_instance.main.port
      database = aws_db_instance.main.db_name
      username = aws_db_instance.main.username
      # password should be retrieved from secrets manager
      endpoint = aws_db_instance.main.endpoint
    }
    redis = {
      host = aws_elasticache_replication_group.redis.primary_endpoint_address
      port = aws_elasticache_replication_group.redis.port
      # auth_token should be retrieved from secrets manager
    }
    opensearch = {
      endpoint = "https://${aws_opensearch_domain.main.endpoint}"
      username = "admin"
      # password should be retrieved from secrets manager
      kibana_endpoint = "https://${aws_opensearch_domain.main.kibana_endpoint}"
      dashboard_endpoint = "https://${aws_opensearch_domain.main.dashboard_endpoint}"
    }
  }
  sensitive = true
}

#################################################
# Security Group IDs for Reference
#################################################

output "security_group_ids" {
  description = "Security group IDs for database access"
  value = {
    rds        = aws_security_group.rds.id
    redis      = aws_security_group.redis.id
    opensearch = aws_security_group.opensearch.id
  }
}

#################################################
# Database Configuration Summary
#################################################

output "database_summary" {
  description = "Summary of database configurations"
  value = {
    rds = {
      engine         = var.rds_config.engine
      engine_version = var.rds_config.engine_version
      instance_class = var.rds_config.instance_class
      multi_az       = var.enable_multi_az
      encrypted      = true
      backup_enabled = var.rds_config.backup_retention_period > 0
    }
    redis = {
      node_type          = var.redis_config.node_type
      num_cache_clusters = var.redis_config.num_cache_clusters
      encrypted_at_rest  = var.redis_config.at_rest_encryption_enabled
      encrypted_in_transit = var.redis_config.transit_encryption_enabled
      auth_enabled       = var.redis_config.auth_token_enabled
    }
    opensearch = {
      engine_version    = var.opensearch_config.engine_version
      instance_type     = var.opensearch_config.instance_type
      instance_count    = var.opensearch_config.instance_count
      encrypted         = var.opensearch_config.encrypt_at_rest.enabled
      https_enforced    = var.opensearch_config.domain_endpoint_options.enforce_https
    }
  }
}

#################################################
# Cost Estimation
#################################################

output "estimated_monthly_costs" {
  description = "Estimated monthly costs for databases"
  value = {
    note = "Costs vary by region, usage patterns, and specific configurations"
    rds = "Based on instance class ${var.rds_config.instance_class} and storage ${var.rds_config.allocated_storage}GB"
    redis = "Based on node type ${var.redis_config.node_type} with ${var.redis_config.num_cache_clusters} nodes"
    opensearch = "Based on ${var.opensearch_config.instance_count}x ${var.opensearch_config.instance_type} instances"
  }
}

#################################################
# Health Check Information
#################################################

output "health_check_endpoints" {
  description = "Health check endpoints for databases"
  value = {
    rds = {
      command = "mysqladmin ping -h ${aws_db_instance.main.address} -P ${aws_db_instance.main.port} -u ${aws_db_instance.main.username} -p"
    }
    redis = {
      command = "redis-cli -h ${aws_elasticache_replication_group.redis.primary_endpoint_address} -p ${aws_elasticache_replication_group.redis.port} ping"
    }
    opensearch = {
      endpoint = "https://${aws_opensearch_domain.main.endpoint}/_cluster/health"
    }
  }
  sensitive = true
}