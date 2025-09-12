# Container Registry Module
# Creates ECR repositories with security hardening, lifecycle policies, and access controls

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

# Data sources
data "aws_caller_identity" "current" {}
data "aws_partition" "current" {}
data "aws_region" "current" {}

# KMS key for ECR encryption
resource "aws_kms_key" "ecr" {
  description             = "ECR encryption key for ${var.name_prefix}"
  deletion_window_in_days = 7
  enable_key_rotation     = true

  policy = jsonencode({
    Version = "2012-10-17"
    Id      = "key-policy-ecr"
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
        Sid    = "Allow ECR service"
        Effect = "Allow"
        Principal = {
          Service = "ecr.amazonaws.com"
        }
        Action = [
          "kms:Decrypt",
          "kms:GenerateDataKey"
        ]
        Resource = "*"
      }
    ]
  })

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-ecr-kms-key"
  })
}

resource "aws_kms_alias" "ecr" {
  name          = "alias/${var.name_prefix}-ecr"
  target_key_id = aws_kms_key.ecr.key_id
}

# ECR Repositories
resource "aws_ecr_repository" "this" {
  for_each = toset(var.repositories)

  name                 = "${var.name_prefix}-${each.value}"
  image_tag_mutability = var.image_tag_mutability
  force_delete         = var.force_delete_repository

  image_scanning_configuration {
    scan_on_push = var.enable_image_scanning
  }

  encryption_configuration {
    encryption_type = var.encryption_type
    kms_key         = var.encryption_type == "KMS" ? aws_kms_key.ecr.arn : null
  }

  tags = merge(var.tags, {
    Name        = "${var.name_prefix}-${each.value}"
    Repository  = each.value
  })
}

# Repository Lifecycle Policies
resource "aws_ecr_lifecycle_policy" "this" {
  for_each = var.enable_lifecycle_policy ? toset(var.repositories) : []

  repository = aws_ecr_repository.this[each.value].name

  policy = jsonencode({
    rules = concat(
      var.lifecycle_policy_rules,
      [
        # Default rule to keep last N production images
        {
          rulePriority = 100
          description  = "Keep last ${var.max_production_images} production images"
          selection = {
            tagStatus     = "tagged"
            tagPrefixList = var.production_tag_prefixes
            countType     = "imageCountMoreThan"
            countNumber   = var.max_production_images
          }
          action = {
            type = "expire"
          }
        },
        # Default rule to expire untagged images
        {
          rulePriority = 200
          description  = "Delete untagged images after ${var.untagged_image_expiry_days} days"
          selection = {
            tagStatus   = "untagged"
            countType   = "sinceImagePushed"
            countUnit   = "days"
            countNumber = var.untagged_image_expiry_days
          }
          action = {
            type = "expire"
          }
        }
      ]
    )
  })
}

# Repository Policies for cross-account access
resource "aws_ecr_repository_policy" "this" {
  for_each = var.enable_cross_account_access ? toset(var.repositories) : []

  repository = aws_ecr_repository.this[each.value].name

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = concat(
      # Allow current account full access
      [
        {
          Sid    = "AllowCurrentAccount"
          Effect = "Allow"
          Principal = {
            AWS = "arn:${data.aws_partition.current.partition}:iam::${data.aws_caller_identity.current.account_id}:root"
          }
          Action = [
            "ecr:GetDownloadUrlForLayer",
            "ecr:BatchGetImage",
            "ecr:BatchCheckLayerAvailability",
            "ecr:PutImage",
            "ecr:InitiateLayerUpload",
            "ecr:UploadLayerPart",
            "ecr:CompleteLayerUpload",
            "ecr:DescribeRepositories",
            "ecr:GetRepositoryPolicy",
            "ecr:ListImages",
            "ecr:DeleteRepository",
            "ecr:BatchDeleteImage",
            "ecr:SetRepositoryPolicy",
            "ecr:DeleteRepositoryPolicy"
          ]
        }
      ],
      # Allow cross-account pull access
      var.cross_account_ids != null ? [
        {
          Sid    = "AllowCrossAccountPull"
          Effect = "Allow"
          Principal = {
            AWS = [for account in var.cross_account_ids : "arn:${data.aws_partition.current.partition}:iam::${account}:root"]
          }
          Action = [
            "ecr:GetDownloadUrlForLayer",
            "ecr:BatchGetImage",
            "ecr:BatchCheckLayerAvailability"
          ]
        }
      ] : [],
      # Allow specific IAM roles access
      var.allowed_iam_roles != null ? [
        {
          Sid    = "AllowSpecificRoles"
          Effect = "Allow"
          Principal = {
            AWS = var.allowed_iam_roles
          }
          Action = [
            "ecr:GetDownloadUrlForLayer",
            "ecr:BatchGetImage",
            "ecr:BatchCheckLayerAvailability",
            "ecr:PutImage",
            "ecr:InitiateLayerUpload",
            "ecr:UploadLayerPart",
            "ecr:CompleteLayerUpload"
          ]
        }
      ] : []
    )
  })
}

# Registry Scanning Configuration
resource "aws_ecr_registry_scanning_configuration" "this" {
  count = var.enable_enhanced_scanning ? 1 : 0

  scan_type = "ENHANCED"

  dynamic "rule" {
    for_each = var.scanning_rules
    content {
      scan_frequency = rule.value.scan_frequency
      repository_filter {
        filter      = rule.value.repository_filter
        filter_type = rule.value.filter_type
      }
    }
  }
}

# Registry Replication Configuration
resource "aws_ecr_replication_configuration" "this" {
  count = var.enable_replication ? 1 : 0

  replication_configuration {
    dynamic "rule" {
      for_each = var.replication_rules
      content {
        dynamic "destination" {
          for_each = rule.value.destinations
          content {
            region      = destination.value.region
            registry_id = destination.value.registry_id
          }
        }
      }
    }
  }
}

# IAM role for ECR push/pull from EKS
resource "aws_iam_role" "ecr_access" {
  count = var.create_ecr_access_role ? 1 : 0
  name  = "${var.name_prefix}-ecr-access-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Service = "ec2.amazonaws.com"
        }
        Action = "sts:AssumeRole"
      },
      {
        Effect = "Allow"
        Principal = {
          Federated = var.oidc_provider_arn
        }
        Action = "sts:AssumeRoleWithWebIdentity"
        Condition = var.oidc_provider_arn != null ? {
          StringEquals = {
            "${replace(var.oidc_provider_arn, "/^.*://", "")}:aud": "sts.amazonaws.com"
          }
        } : {}
      }
    ]
  })

  tags = var.tags
}

# IAM policy for ECR access
resource "aws_iam_role_policy" "ecr_access" {
  count = var.create_ecr_access_role ? 1 : 0
  name  = "${var.name_prefix}-ecr-access-policy"
  role  = aws_iam_role.ecr_access[0].id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "ecr:BatchCheckLayerAvailability",
          "ecr:GetDownloadUrlForLayer",
          "ecr:BatchGetImage"
        ]
        Resource = [for repo in aws_ecr_repository.this : repo.arn]
      },
      {
        Effect = "Allow"
        Action = [
          "ecr:GetAuthorizationToken"
        ]
        Resource = "*"
      }
    ]
  })
}

# IAM policy for ECR push access (for CI/CD)
resource "aws_iam_policy" "ecr_push_policy" {
  count = var.create_push_policy ? 1 : 0
  name  = "${var.name_prefix}-ecr-push-policy"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "ecr:GetAuthorizationToken"
        ]
        Resource = "*"
      },
      {
        Effect = "Allow"
        Action = [
          "ecr:BatchCheckLayerAvailability",
          "ecr:GetDownloadUrlForLayer",
          "ecr:BatchGetImage",
          "ecr:PutImage",
          "ecr:InitiateLayerUpload",
          "ecr:UploadLayerPart",
          "ecr:CompleteLayerUpload"
        ]
        Resource = [for repo in aws_ecr_repository.this : repo.arn]
      }
    ]
  })

  tags = var.tags
}

# CloudWatch Log Group for ECR events
resource "aws_cloudwatch_log_group" "ecr_events" {
  count             = var.enable_cloudwatch_logging ? 1 : 0
  name              = "/aws/ecr/${var.name_prefix}"
  retention_in_days = var.log_retention_days
  kms_key_id        = aws_kms_key.ecr.arn

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-ecr-logs"
  })
}

# EventBridge rule for ECR scan results
resource "aws_cloudwatch_event_rule" "ecr_scan_results" {
  count = var.enable_scan_result_notifications ? 1 : 0
  name  = "${var.name_prefix}-ecr-scan-results"

  event_pattern = jsonencode({
    source        = ["aws.ecr"]
    detail-type   = ["ECR Image Scan"]
    detail = {
      scan-status = ["COMPLETE"]
      repository-name = [for repo in var.repositories : "${var.name_prefix}-${repo}"]
    }
  })

  tags = var.tags
}

# SNS topic for notifications
resource "aws_sns_topic" "ecr_notifications" {
  count             = var.enable_scan_result_notifications ? 1 : 0
  name              = "${var.name_prefix}-ecr-notifications"
  kms_master_key_id = aws_kms_key.ecr.arn

  tags = var.tags
}

# EventBridge target for SNS
resource "aws_cloudwatch_event_target" "sns" {
  count     = var.enable_scan_result_notifications ? 1 : 0
  rule      = aws_cloudwatch_event_rule.ecr_scan_results[0].name
  target_id = "SendToSNS"
  arn       = aws_sns_topic.ecr_notifications[0].arn
}

# SNS topic policy
resource "aws_sns_topic_policy" "ecr_notifications" {
  count = var.enable_scan_result_notifications ? 1 : 0
  arn   = aws_sns_topic.ecr_notifications[0].arn

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Service = "events.amazonaws.com"
        }
        Action   = "sns:Publish"
        Resource = aws_sns_topic.ecr_notifications[0].arn
        Condition = {
          StringEquals = {
            "aws:SourceAccount" = data.aws_caller_identity.current.account_id
          }
        }
      }
    ]
  })
}