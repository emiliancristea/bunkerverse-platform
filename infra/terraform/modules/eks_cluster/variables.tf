# EKS Cluster Module Variables

variable "name_prefix" {
  description = "Prefix for naming resources"
  type        = string

  validation {
    condition     = can(regex("^[a-z][a-z0-9-]*$", var.name_prefix))
    error_message = "Name prefix must start with a letter and contain only lowercase letters, numbers, and hyphens."
  }
}

variable "vpc_id" {
  description = "ID of the VPC where the cluster will be created"
  type        = string
}

variable "private_subnet_ids" {
  description = "List of private subnet IDs for the node groups"
  type        = list(string)
}

variable "public_subnet_ids" {
  description = "List of public subnet IDs for the cluster endpoint"
  type        = list(string)
}

variable "cluster_version" {
  description = "Kubernetes version for the EKS cluster"
  type        = string
  default     = "1.28"

  validation {
    condition     = can(regex("^1\\.(2[6-9]|[3-9][0-9])$", var.cluster_version))
    error_message = "Cluster version must be 1.26 or higher."
  }
}

variable "cluster_endpoint_private" {
  description = "Enable private API server endpoint"
  type        = bool
  default     = false
}

variable "cluster_endpoint_public" {
  description = "Enable public API server endpoint"
  type        = bool
  default     = true
}

variable "cluster_endpoint_public_access_cidrs" {
  description = "List of CIDR blocks that can access the public API server endpoint"
  type        = list(string)
  default     = ["0.0.0.0/0"]
}

variable "node_groups" {
  description = "Map of node group configurations"
  type = map(object({
    instance_types = list(string)
    capacity_type  = string
    ami_id         = optional(string)
    disk_size      = optional(number, 50)

    scaling_config = object({
      desired_size = number
      max_size     = number
      min_size     = number
    })

    update_config = object({
      max_unavailable_percentage = optional(number, 25)
    })

    labels = optional(map(string), {})
    taints = optional(list(object({
      key    = string
      value  = string
      effect = string
    })), [])

    bootstrap_arguments = optional(string, "")
  }))

  validation {
    condition = alltrue([
      for k, v in var.node_groups : contains(["ON_DEMAND", "SPOT"], v.capacity_type)
    ])
    error_message = "Capacity type must be either ON_DEMAND or SPOT."
  }

  validation {
    condition = alltrue([
      for k, v in var.node_groups : v.scaling_config.desired_size >= v.scaling_config.min_size && v.scaling_config.desired_size <= v.scaling_config.max_size
    ])
    error_message = "Desired size must be between min_size and max_size."
  }
}

variable "cluster_addons" {
  description = "Map of cluster addon configurations"
  type = map(object({
    addon_version            = optional(string)
    resolve_conflicts        = optional(string, "OVERWRITE")
    service_account_role_arn = optional(string)
  }))
  default = {}
}

variable "enable_aws_load_balancer_controller" {
  description = "Enable AWS Load Balancer Controller IAM role"
  type        = bool
  default     = true
}

variable "enable_ebs_csi_driver" {
  description = "Enable EBS CSI driver IAM role"
  type        = bool
  default     = true
}

variable "cluster_security_group_additional_rules" {
  description = "Additional security group rules to add to the cluster security group"
  type = map(object({
    description = string
    protocol    = string
    from_port   = number
    to_port     = number
    type        = string
    cidr_blocks = optional(list(string))
    source_security_group_id = optional(string)
  }))
  default = {}
}

variable "node_security_group_additional_rules" {
  description = "Additional security group rules to add to the node security group"
  type = map(object({
    description = string
    protocol    = string
    from_port   = number
    to_port     = number
    type        = string
    cidr_blocks = optional(list(string))
    source_security_group_id = optional(string)
  }))
  default = {}
}

variable "cluster_encryption_config_enabled" {
  description = "Enable envelope encryption of Kubernetes secrets using KMS"
  type        = bool
  default     = true
}

variable "cluster_encryption_config_kms_key_id" {
  description = "KMS key ID for envelope encryption. If not specified, a key will be created."
  type        = string
  default     = ""
}

variable "cluster_log_retention_in_days" {
  description = "Number of days to retain log events"
  type        = number
  default     = 30

  validation {
    condition = contains([
      1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1827, 3653
    ], var.cluster_log_retention_in_days)
    error_message = "Log retention period must be one of the allowed values."
  }
}

variable "enable_irsa" {
  description = "Enable IAM Roles for Service Accounts (IRSA)"
  type        = bool
  default     = true
}

variable "openid_connect_audiences" {
  description = "List of audiences for the OpenID Connect identity provider"
  type        = list(string)
  default     = ["sts.amazonaws.com"]
}

variable "create_cloudwatch_log_group" {
  description = "Create CloudWatch log group for cluster logs"
  type        = bool
  default     = true
}

variable "manage_aws_auth_configmap" {
  description = "Whether to manage the aws-auth configmap"
  type        = bool
  default     = false
}

variable "aws_auth_roles" {
  description = "List of role maps to add to the aws-auth configmap"
  type = list(object({
    rolearn  = string
    username = string
    groups   = list(string)
  }))
  default = []
}

variable "aws_auth_users" {
  description = "List of user maps to add to the aws-auth configmap"
  type = list(object({
    userarn  = string
    username = string
    groups   = list(string)
  }))
  default = []
}

variable "aws_auth_accounts" {
  description = "List of account maps to add to the aws-auth configmap"
  type        = list(string)
  default     = []
}

variable "enable_cluster_creator_admin_permissions" {
  description = "Enable cluster creator admin permissions in aws-auth configmap"
  type        = bool
  default     = true
}

variable "tags" {
  description = "A map of tags to assign to the resources"
  type        = map(string)
  default     = {}
}