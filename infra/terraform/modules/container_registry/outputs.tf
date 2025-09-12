# Container Registry Module Outputs

# Repository Information
output "repository_urls" {
  description = "Map of repository names to their URLs"
  value       = { for k, v in aws_ecr_repository.this : k => v.repository_url }
}

output "repository_arns" {
  description = "Map of repository names to their ARNs"
  value       = { for k, v in aws_ecr_repository.this : k => v.arn }
}

output "repository_registry_ids" {
  description = "Map of repository names to their registry IDs"
  value       = { for k, v in aws_ecr_repository.this : k => v.registry_id }
}

output "repository_names" {
  description = "List of repository names created"
  value       = [for repo in aws_ecr_repository.this : repo.name]
}

# Registry Information
output "registry_id" {
  description = "The registry ID where repositories were created"
  value       = aws_ecr_repository.this[var.repositories[0]].registry_id
}

output "registry_uri" {
  description = "The URI of the registry"
  value       = "${data.aws_caller_identity.current.account_id}.dkr.ecr.${data.aws_region.current.name}.amazonaws.com"
}

# Encryption
output "kms_key_id" {
  description = "The globally unique identifier for the KMS key used for ECR encryption"
  value       = var.encryption_type == "KMS" ? aws_kms_key.ecr.key_id : null
}

output "kms_key_arn" {
  description = "The Amazon Resource Name (ARN) of the KMS key used for ECR encryption"
  value       = var.encryption_type == "KMS" ? aws_kms_key.ecr.arn : null
}

output "kms_alias_name" {
  description = "The display name of the KMS key alias"
  value       = var.encryption_type == "KMS" ? aws_kms_alias.ecr.name : null
}

# IAM Roles and Policies
output "ecr_access_role_arn" {
  description = "ARN of the IAM role for ECR access"
  value       = var.create_ecr_access_role ? aws_iam_role.ecr_access[0].arn : null
}

output "ecr_access_role_name" {
  description = "Name of the IAM role for ECR access"
  value       = var.create_ecr_access_role ? aws_iam_role.ecr_access[0].name : null
}

output "ecr_push_policy_arn" {
  description = "ARN of the IAM policy for ECR push access"
  value       = var.create_push_policy ? aws_iam_policy.ecr_push_policy[0].arn : null
}

output "ecr_push_policy_name" {
  description = "Name of the IAM policy for ECR push access"
  value       = var.create_push_policy ? aws_iam_policy.ecr_push_policy[0].name : null
}

# Scanning Configuration
output "scanning_configuration" {
  description = "ECR scanning configuration"
  value = var.enable_enhanced_scanning ? {
    scan_type = aws_ecr_registry_scanning_configuration.this[0].scan_type
    rules     = aws_ecr_registry_scanning_configuration.this[0].rule
  } : null
}

# Replication Configuration
output "replication_configuration" {
  description = "ECR replication configuration"
  value = var.enable_replication ? {
    rules = aws_ecr_replication_configuration.this[0].replication_configuration
  } : null
}

# Monitoring and Notifications
output "cloudwatch_log_group_name" {
  description = "Name of the CloudWatch log group for ECR events"
  value       = var.enable_cloudwatch_logging ? aws_cloudwatch_log_group.ecr_events[0].name : null
}

output "cloudwatch_log_group_arn" {
  description = "ARN of the CloudWatch log group for ECR events"
  value       = var.enable_cloudwatch_logging ? aws_cloudwatch_log_group.ecr_events[0].arn : null
}

output "sns_topic_arn" {
  description = "ARN of the SNS topic for ECR notifications"
  value       = var.enable_scan_result_notifications ? aws_sns_topic.ecr_notifications[0].arn : null
}

output "sns_topic_name" {
  description = "Name of the SNS topic for ECR notifications"
  value       = var.enable_scan_result_notifications ? aws_sns_topic.ecr_notifications[0].name : null
}

output "eventbridge_rule_name" {
  description = "Name of the EventBridge rule for ECR scan results"
  value       = var.enable_scan_result_notifications ? aws_cloudwatch_event_rule.ecr_scan_results[0].name : null
}

# Security Information
output "repository_policies" {
  description = "Repository policies for cross-account access"
  value = var.enable_cross_account_access ? {
    for k, v in aws_ecr_repository_policy.this : k => v.policy
  } : {}
}

# Docker Login Command
output "docker_login_command" {
  description = "AWS CLI command to log in to ECR"
  value       = "aws ecr get-login-password --region ${data.aws_region.current.name} | docker login --username AWS --password-stdin ${data.aws_caller_identity.current.account_id}.dkr.ecr.${data.aws_region.current.name}.amazonaws.com"
}

# Repository Details
output "repositories_detailed" {
  description = "Detailed information about all repositories"
  value = {
    for k, v in aws_ecr_repository.this : k => {
      name                 = v.name
      arn                  = v.arn
      registry_id          = v.registry_id
      repository_url       = v.repository_url
      image_tag_mutability = v.image_tag_mutability
      encryption_configuration = v.encryption_configuration
      image_scanning_configuration = v.image_scanning_configuration
      tags                 = v.tags
    }
  }
}

# Cost Information
output "estimated_monthly_cost" {
  description = "Estimated monthly cost for ECR repositories (basic calculation)"
  value = {
    storage_gb_per_month = "Variable based on image storage"
    data_transfer_gb_per_month = "Variable based on pulls/pushes"
    note = "Actual costs depend on usage patterns and data transfer"
  }
}

# Security Recommendations
output "security_recommendations" {
  description = "Security recommendations for ECR usage"
  value = {
    encryption = var.encryption_type == "KMS" ? "✓ KMS encryption enabled" : "⚠ Consider enabling KMS encryption"
    scanning = var.enable_image_scanning ? "✓ Image scanning enabled" : "⚠ Consider enabling image scanning"
    lifecycle_policy = var.enable_lifecycle_policy ? "✓ Lifecycle policy configured" : "⚠ Consider configuring lifecycle policies"
    cross_account_access = var.enable_cross_account_access ? "✓ Cross-account access configured" : "ℹ Cross-account access not configured"
  }
}

# Quick Access URLs
output "ecr_console_urls" {
  description = "Direct links to ECR repositories in AWS Console"
  value = {
    for k, v in aws_ecr_repository.this : k => 
    "https://console.aws.amazon.com/ecr/repositories/private/${data.aws_caller_identity.current.account_id}/${v.name}?region=${data.aws_region.current.name}"
  }
}