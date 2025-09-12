# Container Registry Module Variables

variable "name_prefix" {
  description = "Prefix for naming resources"
  type        = string

  validation {
    condition     = can(regex("^[a-z][a-z0-9-]*$", var.name_prefix))
    error_message = "Name prefix must start with a letter and contain only lowercase letters, numbers, and hyphens."
  }
}

variable "repositories" {
  description = "List of repository names to create"
  type        = list(string)

  validation {
    condition     = length(var.repositories) > 0
    error_message = "At least one repository must be specified."
  }

  validation {
    condition = alltrue([
      for repo in var.repositories : can(regex("^[a-z][a-z0-9-]*$", repo))
    ])
    error_message = "Repository names must start with a letter and contain only lowercase letters, numbers, and hyphens."
  }
}

variable "image_tag_mutability" {
  description = "The tag mutability setting for the repository (MUTABLE or IMMUTABLE)"
  type        = string
  default     = "MUTABLE"

  validation {
    condition     = contains(["MUTABLE", "IMMUTABLE"], var.image_tag_mutability)
    error_message = "Image tag mutability must be either MUTABLE or IMMUTABLE."
  }
}

variable "enable_image_scanning" {
  description = "Enable vulnerability scanning on image push"
  type        = bool
  default     = true
}

variable "encryption_type" {
  description = "The encryption type for the repository (AES256 or KMS)"
  type        = string
  default     = "KMS"

  validation {
    condition     = contains(["AES256", "KMS"], var.encryption_type)
    error_message = "Encryption type must be either AES256 or KMS."
  }
}

variable "force_delete_repository" {
  description = "Allow deletion of non-empty repositories"
  type        = bool
  default     = false
}

# Lifecycle Policy Configuration
variable "enable_lifecycle_policy" {
  description = "Enable lifecycle policy for repositories"
  type        = bool
  default     = true
}

variable "lifecycle_policy_rules" {
  description = "Custom lifecycle policy rules"
  type = list(object({
    rulePriority = number
    description  = string
    selection = object({
      tagStatus     = string
      tagPrefixList = optional(list(string))
      countType     = string
      countNumber   = optional(number)
      countUnit     = optional(string)
    })
    action = object({
      type = string
    })
  }))
  default = []
}

variable "max_production_images" {
  description = "Maximum number of production images to keep"
  type        = number
  default     = 10

  validation {
    condition     = var.max_production_images > 0 && var.max_production_images <= 1000
    error_message = "Max production images must be between 1 and 1000."
  }
}

variable "production_tag_prefixes" {
  description = "List of tag prefixes considered as production images"
  type        = list(string)
  default     = ["v", "release", "prod"]
}

variable "untagged_image_expiry_days" {
  description = "Number of days after which untagged images expire"
  type        = number
  default     = 1

  validation {
    condition     = var.untagged_image_expiry_days >= 1 && var.untagged_image_expiry_days <= 365
    error_message = "Untagged image expiry days must be between 1 and 365."
  }
}

# Access Control Configuration
variable "enable_cross_account_access" {
  description = "Enable cross-account access to repositories"
  type        = bool
  default     = false
}

variable "cross_account_ids" {
  description = "List of AWS account IDs allowed to pull images"
  type        = list(string)
  default     = null

  validation {
    condition = var.cross_account_ids == null ? true : alltrue([
      for account in var.cross_account_ids : can(regex("^[0-9]{12}$", account))
    ])
    error_message = "Account IDs must be 12-digit strings."
  }
}

variable "allowed_iam_roles" {
  description = "List of IAM role ARNs allowed to access repositories"
  type        = list(string)
  default     = null
}

# IAM Role Configuration
variable "create_ecr_access_role" {
  description = "Create IAM role for ECR access from EKS"
  type        = bool
  default     = true
}

variable "create_push_policy" {
  description = "Create IAM policy for ECR push access (for CI/CD)"
  type        = bool
  default     = true
}

variable "oidc_provider_arn" {
  description = "ARN of the OIDC provider for IRSA"
  type        = string
  default     = null
}

# Enhanced Scanning Configuration
variable "enable_enhanced_scanning" {
  description = "Enable enhanced scanning (Inspector v2)"
  type        = bool
  default     = false
}

variable "scanning_rules" {
  description = "Enhanced scanning rules configuration"
  type = list(object({
    scan_frequency    = string
    repository_filter = string
    filter_type      = string
  }))
  default = []
}

# Replication Configuration
variable "enable_replication" {
  description = "Enable cross-region replication"
  type        = bool
  default     = false
}

variable "replication_rules" {
  description = "Replication rules configuration"
  type = list(object({
    destinations = list(object({
      region      = string
      registry_id = string
    }))
  }))
  default = []
}

# Monitoring and Notifications
variable "enable_cloudwatch_logging" {
  description = "Enable CloudWatch logging for ECR events"
  type        = bool
  default     = true
}

variable "log_retention_days" {
  description = "CloudWatch log retention period in days"
  type        = number
  default     = 30

  validation {
    condition = contains([
      1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1827, 3653
    ], var.log_retention_days)
    error_message = "Log retention period must be one of the allowed values."
  }
}

variable "enable_scan_result_notifications" {
  description = "Enable SNS notifications for scan results"
  type        = bool
  default     = false
}

variable "notification_endpoints" {
  description = "List of notification endpoints (email, SMS, etc.)"
  type        = list(string)
  default     = []
}

# Cost Optimization
variable "enable_cost_optimization" {
  description = "Enable cost optimization features"
  type        = bool
  default     = true
}

variable "enable_multi_arch_support" {
  description = "Enable multi-architecture image support"
  type        = bool
  default     = true
}

# Security Configuration
variable "enable_private_scanning" {
  description = "Enable private vulnerability scanning"
  type        = bool
  default     = true
}

variable "scan_on_push_filters" {
  description = "Filters to apply for scan-on-push"
  type = list(object({
    filter      = string
    filter_type = string
  }))
  default = []
}

variable "vulnerability_severity_threshold" {
  description = "Minimum severity level for vulnerability notifications"
  type        = string
  default     = "MEDIUM"

  validation {
    condition     = contains(["INFORMATIONAL", "LOW", "MEDIUM", "HIGH", "CRITICAL"], var.vulnerability_severity_threshold)
    error_message = "Vulnerability severity threshold must be one of: INFORMATIONAL, LOW, MEDIUM, HIGH, CRITICAL."
  }
}

variable "tags" {
  description = "A map of tags to assign to the resources"
  type        = map(string)
  default     = {}
}