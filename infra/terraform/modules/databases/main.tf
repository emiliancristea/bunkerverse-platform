# Databases Module
# Creates RDS MySQL, ElastiCache Redis, and OpenSearch with security hardening

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.4"
    }
  }
}

# Data sources
data "aws_caller_identity" "current" {}
data "aws_partition" "current" {}
data "aws_region" "current" {}
data "aws_availability_zones" "available" {
  state = "available"
}

# Random passwords for databases
resource "random_password" "rds_password" {
  length  = 32
  special = true
}

resource "random_password" "redis_auth_token" {
  length  = 32
  special = false
  upper   = true
  lower   = true
  numeric = true
}

resource "random_password" "opensearch_master_password" {
  length  = 16
  special = true
}

# KMS key for database encryption
resource "aws_kms_key" "databases" {
  description             = "KMS key for database encryption in ${var.name_prefix}"
  deletion_window_in_days = 7
  enable_key_rotation     = true

  policy = jsonencode({
    Version = "2012-10-17"
    Id      = "key-policy-databases"
    Statement = [
      {
        Sid    = "Enable IAM User Permissions"
        Effect = "Allow"
        Principal = {
          AWS = "arn:${data.aws_partition.current.partition}:iam::${data.aws_caller_identity.current.account_id}:root"
        }
        Action   = "kms:*"
        Resource = "*"
      },
      {
        Sid    = "Allow database services"
        Effect = "Allow"
        Principal = {
          Service = [
            "rds.amazonaws.com",
            "elasticache.amazonaws.com",
            "es.amazonaws.com"
          ]
        }
        Action = [
          "kms:Decrypt",
          "kms:GenerateDataKey",
          "kms:CreateGrant"
        ]
        Resource = "*"
      }
    ]
  })

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-databases-kms-key"
  })
}

resource "aws_kms_alias" "databases" {
  name          = "alias/${var.name_prefix}-databases"
  target_key_id = aws_kms_key.databases.key_id
}

#################################################
# RDS MySQL Configuration
#################################################

# DB Subnet Group
resource "aws_db_subnet_group" "main" {
  name       = "${var.name_prefix}-db-subnet-group"
  subnet_ids = var.database_subnet_ids

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-db-subnet-group"
  })
}

# DB Parameter Group
resource "aws_db_parameter_group" "main" {
  family = "mysql8.0"
  name   = "${var.name_prefix}-mysql-params"

  parameter {
    name  = "innodb_buffer_pool_size"
    value = "{DBInstanceClassMemory*3/4}"
  }

  parameter {
    name  = "slow_query_log"
    value = "1"
  }

  parameter {
    name  = "long_query_time"
    value = "2"
  }

  parameter {
    name  = "log_queries_not_using_indexes"
    value = "1"
  }

  parameter {
    name  = "general_log"
    value = var.rds_config.enabled_cloudwatch_logs_exports != null ? "1" : "0"
  }

  tags = var.tags
}

# Security Group for RDS
resource "aws_security_group" "rds" {
  name_prefix = "${var.name_prefix}-rds-"
  vpc_id      = var.vpc_id
  description = "Security group for RDS MySQL instance"

  ingress {
    description = "MySQL from VPC"
    from_port   = 3306
    to_port     = 3306
    protocol    = "tcp"
    cidr_blocks = [data.aws_vpc.main.cidr_block]
  }

  egress {
    description = "All outbound traffic"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-rds-sg"
  })

  lifecycle {
    create_before_destroy = true
  }
}

data "aws_vpc" "main" {
  id = var.vpc_id
}

# RDS Instance
resource "aws_db_instance" "main" {
  identifier = "${var.name_prefix}-mysql"

  # Engine configuration
  engine         = var.rds_config.engine
  engine_version = var.rds_config.engine_version
  instance_class = var.rds_config.instance_class

  # Storage configuration
  allocated_storage     = var.rds_config.allocated_storage
  max_allocated_storage = var.rds_config.max_allocated_storage
  storage_type          = "gp3"
  storage_encrypted     = true
  kms_key_id           = aws_kms_key.databases.arn

  # Database configuration
  db_name  = var.rds_config.database_name
  username = var.rds_config.username
  password = random_password.rds_password.result
  port     = 3306

  # Network configuration
  db_subnet_group_name   = aws_db_subnet_group.main.name
  vpc_security_group_ids = [aws_security_group.rds.id]
  publicly_accessible    = false

  # Parameter group
  parameter_group_name = aws_db_parameter_group.main.name

  # Backup configuration
  backup_retention_period = var.rds_config.backup_retention_period
  backup_window          = var.rds_config.backup_window
  maintenance_window     = var.rds_config.maintenance_window
  copy_tags_to_snapshot  = true

  # Deletion protection
  deletion_protection = var.rds_config.deletion_protection
  skip_final_snapshot = var.rds_config.skip_final_snapshot

  # Monitoring
  monitoring_interval                   = var.rds_config.monitoring_interval
  monitoring_role_arn                  = var.rds_config.monitoring_interval > 0 ? aws_iam_role.rds_enhanced_monitoring[0].arn : null
  performance_insights_enabled         = var.rds_config.performance_insights_enabled
  performance_insights_kms_key_id     = var.rds_config.performance_insights_enabled ? aws_kms_key.databases.arn : null
  performance_insights_retention_period = var.rds_config.performance_insights_enabled ? 7 : null

  # CloudWatch Logs
  enabled_cloudwatch_logs_exports = var.rds_config.enabled_cloudwatch_logs_exports

  # Multi-AZ
  multi_az = var.enable_multi_az

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-mysql"
  })

  lifecycle {
    ignore_changes = [password]
  }
}

# Enhanced Monitoring Role for RDS
resource "aws_iam_role" "rds_enhanced_monitoring" {
  count = var.rds_config.monitoring_interval > 0 ? 1 : 0
  name  = "${var.name_prefix}-rds-enhanced-monitoring"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Service = "monitoring.rds.amazonaws.com"
        }
        Action = "sts:AssumeRole"
      }
    ]
  })

  tags = var.tags
}

resource "aws_iam_role_policy_attachment" "rds_enhanced_monitoring" {
  count      = var.rds_config.monitoring_interval > 0 ? 1 : 0
  role       = aws_iam_role.rds_enhanced_monitoring[0].name
  policy_arn = "arn:${data.aws_partition.current.partition}:iam::aws:policy/service-role/AmazonRDSEnhancedMonitoringRole"
}

#################################################
# ElastiCache Redis Configuration
#################################################

# ElastiCache Subnet Group
resource "aws_elasticache_subnet_group" "main" {
  name       = "${var.name_prefix}-redis-subnet-group"
  subnet_ids = var.database_subnet_ids

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-redis-subnet-group"
  })
}

# ElastiCache Parameter Group
resource "aws_elasticache_parameter_group" "redis" {
  family = "redis7.x"
  name   = "${var.name_prefix}-redis-params"

  parameter {
    name  = "maxmemory-policy"
    value = "allkeys-lru"
  }

  parameter {
    name  = "timeout"
    value = "300"
  }

  parameter {
    name  = "tcp-keepalive"
    value = "300"
  }

  tags = var.tags
}

# Security Group for Redis
resource "aws_security_group" "redis" {
  name_prefix = "${var.name_prefix}-redis-"
  vpc_id      = var.vpc_id
  description = "Security group for ElastiCache Redis"

  ingress {
    description = "Redis from VPC"
    from_port   = var.redis_config.port
    to_port     = var.redis_config.port
    protocol    = "tcp"
    cidr_blocks = [data.aws_vpc.main.cidr_block]
  }

  egress {
    description = "All outbound traffic"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-redis-sg"
  })

  lifecycle {
    create_before_destroy = true
  }
}

# ElastiCache Replication Group
resource "aws_elasticache_replication_group" "redis" {
  replication_group_id         = "${var.name_prefix}-redis"
  description                  = "Redis cluster for ${var.name_prefix}"
  
  port                        = var.redis_config.port
  parameter_group_name        = aws_elasticache_parameter_group.redis.name
  node_type                   = var.redis_config.node_type
  num_cache_clusters          = var.redis_config.num_cache_clusters
  
  # Network configuration
  subnet_group_name = aws_elasticache_subnet_group.main.name
  security_group_ids = [aws_security_group.redis.id]
  
  # Security configuration
  at_rest_encryption_enabled = var.redis_config.at_rest_encryption_enabled
  transit_encryption_enabled = var.redis_config.transit_encryption_enabled
  auth_token_enabled         = var.redis_config.auth_token_enabled
  auth_token                 = var.redis_config.auth_token_enabled ? random_password.redis_auth_token.result : null
  kms_key_id                 = var.redis_config.at_rest_encryption_enabled ? aws_kms_key.databases.arn : null
  
  # Backup configuration
  snapshot_retention_limit = var.redis_config.snapshot_retention_limit
  snapshot_window         = var.redis_config.snapshot_window
  
  # Maintenance
  maintenance_window = var.redis_config.maintenance_window
  
  # Multi-AZ
  multi_az_enabled = var.enable_multi_az && var.redis_config.num_cache_clusters > 1
  
  # Logging
  log_delivery_configuration {
    destination      = aws_cloudwatch_log_group.redis_slow.name
    destination_type = "cloudwatch-logs"
    log_format       = "text"
    log_type         = "slow-log"
  }

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-redis"
  })

  lifecycle {
    ignore_changes = [auth_token]
  }
}

# CloudWatch Log Groups for Redis
resource "aws_cloudwatch_log_group" "redis_slow" {
  name              = "/aws/elasticache/${var.name_prefix}/redis-slow-log"
  retention_in_days = 30
  kms_key_id        = aws_kms_key.databases.arn

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-redis-slow-log"
  })
}

#################################################
# OpenSearch Configuration
#################################################

# Security Group for OpenSearch
resource "aws_security_group" "opensearch" {
  name_prefix = "${var.name_prefix}-opensearch-"
  vpc_id      = var.vpc_id
  description = "Security group for OpenSearch cluster"

  ingress {
    description = "HTTPS from VPC"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = [data.aws_vpc.main.cidr_block]
  }

  ingress {
    description = "HTTP from VPC"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = [data.aws_vpc.main.cidr_block]
  }

  egress {
    description = "All outbound traffic"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-opensearch-sg"
  })

  lifecycle {
    create_before_destroy = true
  }
}

# IAM Service-Linked Role for OpenSearch
resource "aws_iam_service_linked_role" "opensearch" {
  aws_service_name = "opensearchserverless.amazonaws.com"
  description      = "Service-linked role for OpenSearch"

  lifecycle {
    ignore_changes = [aws_service_name]
  }
}

# OpenSearch Domain
resource "aws_opensearch_domain" "main" {
  domain_name    = "${var.name_prefix}-search"
  engine_version = var.opensearch_config.engine_version

  cluster_config {
    instance_type            = var.opensearch_config.instance_type
    instance_count           = var.opensearch_config.instance_count
    dedicated_master_enabled = var.opensearch_config.dedicated_master_enabled
    master_instance_type     = var.opensearch_config.master_instance_type
    master_instance_count    = var.opensearch_config.master_instance_count
    zone_awareness_enabled   = var.opensearch_config.instance_count > 1

    dynamic "zone_awareness_config" {
      for_each = var.opensearch_config.instance_count > 1 ? [1] : []
      content {
        availability_zone_count = min(var.opensearch_config.instance_count, length(data.aws_availability_zones.available.names))
      }
    }
  }

  # EBS configuration
  ebs_options {
    ebs_enabled = var.opensearch_config.ebs_enabled
    volume_type = var.opensearch_config.volume_type
    volume_size = var.opensearch_config.volume_size
  }

  # VPC configuration
  vpc_options {
    subnet_ids         = var.opensearch_config.instance_count > 1 ? var.database_subnet_ids : [var.database_subnet_ids[0]]
    security_group_ids = [aws_security_group.opensearch.id]
  }

  # Encryption configuration
  encrypt_at_rest {
    enabled    = var.opensearch_config.encrypt_at_rest.enabled
    kms_key_id = var.opensearch_config.encrypt_at_rest.enabled ? aws_kms_key.databases.arn : null
  }

  node_to_node_encryption {
    enabled = var.opensearch_config.node_to_node_encryption.enabled
  }

  domain_endpoint_options {
    enforce_https       = var.opensearch_config.domain_endpoint_options.enforce_https
    tls_security_policy = var.opensearch_config.domain_endpoint_options.tls_security_policy
  }

  # Advanced security options
  advanced_security_options {
    enabled                        = true
    anonymous_auth_enabled        = false
    internal_user_database_enabled = true
    master_user_options {
      master_user_name     = "admin"
      master_user_password = random_password.opensearch_master_password.result
    }
  }

  # Logging configuration
  log_publishing_options {
    enabled                  = true
    log_type                = "INDEX_SLOW_LOGS"
    cloudwatch_log_group_arn = "${aws_cloudwatch_log_group.opensearch_index_slow.arn}:*"
  }

  log_publishing_options {
    enabled                  = true
    log_type                = "SEARCH_SLOW_LOGS"
    cloudwatch_log_group_arn = "${aws_cloudwatch_log_group.opensearch_search_slow.arn}:*"
  }

  log_publishing_options {
    enabled                  = true
    log_type                = "ES_APPLICATION_LOGS"
    cloudwatch_log_group_arn = "${aws_cloudwatch_log_group.opensearch_application.arn}:*"
  }

  # Snapshot configuration
  snapshot_options {
    automated_snapshot_start_hour = 2
  }

  # Advanced options
  advanced_options = {
    "rest.action.multi.allow_explicit_index" = "true"
    "indices.fielddata.cache.size"            = "20"
    "indices.query.bool.max_clause_count"     = "1024"
  }

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-opensearch"
  })

  depends_on = [aws_iam_service_linked_role.opensearch]

  lifecycle {
    ignore_changes = [advanced_security_options[0].master_user_options[0].master_user_password]
  }
}

# CloudWatch Log Groups for OpenSearch
resource "aws_cloudwatch_log_group" "opensearch_index_slow" {
  name              = "/aws/opensearch/domains/${var.name_prefix}-search/index-slow"
  retention_in_days = 30
  kms_key_id        = aws_kms_key.databases.arn

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-opensearch-index-slow"
  })
}

resource "aws_cloudwatch_log_group" "opensearch_search_slow" {
  name              = "/aws/opensearch/domains/${var.name_prefix}-search/search-slow"
  retention_in_days = 30
  kms_key_id        = aws_kms_key.databases.arn

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-opensearch-search-slow"
  })
}

resource "aws_cloudwatch_log_group" "opensearch_application" {
  name              = "/aws/opensearch/domains/${var.name_prefix}-search/application"
  retention_in_days = 30
  kms_key_id        = aws_kms_key.databases.arn

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-opensearch-application"
  })
}

# IAM policy for OpenSearch logs
resource "aws_cloudwatch_log_resource_policy" "opensearch" {
  policy_name = "${var.name_prefix}-opensearch-log-policy"

  policy_document = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Service = "es.amazonaws.com"
        }
        Action = [
          "logs:PutLogEvents",
          "logs:CreateLogGroup",
          "logs:CreateLogStream"
        ]
        Resource = [
          "${aws_cloudwatch_log_group.opensearch_index_slow.arn}:*",
          "${aws_cloudwatch_log_group.opensearch_search_slow.arn}:*",
          "${aws_cloudwatch_log_group.opensearch_application.arn}:*"
        ]
      }
    ]
  })
}

#################################################
# CloudWatch Alarms for Database Monitoring
#################################################

# RDS CPU Utilization Alarm
resource "aws_cloudwatch_metric_alarm" "rds_cpu_utilization" {
  alarm_name          = "${var.name_prefix}-rds-cpu-utilization"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "2"
  metric_name        = "CPUUtilization"
  namespace          = "AWS/RDS"
  period             = "300"
  statistic          = "Average"
  threshold          = "80"
  alarm_description  = "This metric monitors RDS CPU utilization"
  alarm_actions      = var.cloudwatch_alarm_actions

  dimensions = {
    DBInstanceIdentifier = aws_db_instance.main.id
  }

  tags = var.tags
}

# RDS Database Connections Alarm
resource "aws_cloudwatch_metric_alarm" "rds_database_connections" {
  alarm_name          = "${var.name_prefix}-rds-database-connections"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "2"
  metric_name        = "DatabaseConnections"
  namespace          = "AWS/RDS"
  period             = "300"
  statistic          = "Average"
  threshold          = "80"
  alarm_description  = "This metric monitors RDS database connections"
  alarm_actions      = var.cloudwatch_alarm_actions

  dimensions = {
    DBInstanceIdentifier = aws_db_instance.main.id
  }

  tags = var.tags
}

# Redis CPU Utilization Alarm
resource "aws_cloudwatch_metric_alarm" "redis_cpu_utilization" {
  alarm_name          = "${var.name_prefix}-redis-cpu-utilization"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "2"
  metric_name        = "CPUUtilization"
  namespace          = "AWS/ElastiCache"
  period             = "300"
  statistic          = "Average"
  threshold          = "80"
  alarm_description  = "This metric monitors Redis CPU utilization"
  alarm_actions      = var.cloudwatch_alarm_actions

  dimensions = {
    CacheClusterId = "${aws_elasticache_replication_group.redis.replication_group_id}-001"
  }

  tags = var.tags
}

# OpenSearch Cluster Status Alarm
resource "aws_cloudwatch_metric_alarm" "opensearch_cluster_status" {
  alarm_name          = "${var.name_prefix}-opensearch-cluster-status"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "1"
  metric_name        = "ClusterStatus.red"
  namespace          = "AWS/ES"
  period             = "60"
  statistic          = "Maximum"
  threshold          = "0"
  alarm_description  = "This metric monitors OpenSearch cluster status"
  alarm_actions      = var.cloudwatch_alarm_actions
  treat_missing_data = "breaching"

  dimensions = {
    DomainName = aws_opensearch_domain.main.domain_name
    ClientId   = data.aws_caller_identity.current.account_id
  }

  tags = var.tags
}