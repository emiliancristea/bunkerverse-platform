# EKS Cluster Module Outputs

# Cluster information
output "cluster_id" {
  description = "The ID of the EKS cluster"
  value       = aws_eks_cluster.this.id
}

output "cluster_name" {
  description = "The name of the EKS cluster"
  value       = aws_eks_cluster.this.name
}

output "cluster_arn" {
  description = "The Amazon Resource Name (ARN) of the cluster"
  value       = aws_eks_cluster.this.arn
}

output "cluster_endpoint" {
  description = "Endpoint for the EKS control plane"
  value       = aws_eks_cluster.this.endpoint
}

output "cluster_version" {
  description = "The Kubernetes version for the EKS cluster"
  value       = aws_eks_cluster.this.version
}

output "cluster_platform_version" {
  description = "Platform version for the EKS cluster"
  value       = aws_eks_cluster.this.platform_version
}

output "cluster_status" {
  description = "Status of the EKS cluster. One of `CREATING`, `ACTIVE`, `DELETING`, `FAILED`"
  value       = aws_eks_cluster.this.status
}

# Cluster authentication
output "cluster_certificate_authority_data" {
  description = "Base64 encoded certificate data required to communicate with the cluster"
  value       = aws_eks_cluster.this.certificate_authority[0].data
}

output "cluster_token" {
  description = "Token for authenticating with the cluster"
  value       = data.aws_eks_cluster_auth.cluster.token
  sensitive   = true
}

# OIDC Provider
output "cluster_oidc_issuer_url" {
  description = "The URL on the EKS cluster for the OpenID Connect identity provider"
  value       = aws_eks_cluster.this.identity[0].oidc[0].issuer
}

output "oidc_provider_arn" {
  description = "The ARN of the OIDC Provider if enabled"
  value       = var.enable_irsa ? aws_iam_openid_connect_provider.cluster.arn : null
}

# Security Groups
output "cluster_security_group_id" {
  description = "Security group ID attached to the EKS cluster"
  value       = aws_security_group.cluster.id
}

output "cluster_security_group_arn" {
  description = "Amazon Resource Name (ARN) of the cluster security group"
  value       = aws_security_group.cluster.arn
}

output "node_security_group_id" {
  description = "ID of the node shared security group"
  value       = aws_security_group.node_group.id
}

output "node_security_group_arn" {
  description = "Amazon Resource Name (ARN) of the node shared security group"
  value       = aws_security_group.node_group.arn
}

# Node Groups
output "node_groups" {
  description = "Map of attribute maps for all EKS node groups created"
  value = {
    for k, v in aws_eks_node_group.this : k => {
      arn               = v.arn
      capacity_type     = v.capacity_type
      node_group_name   = v.node_group_name
      status            = v.status
      instance_types    = v.instance_types
      scaling_config    = v.scaling_config
      update_config     = v.update_config
      labels            = v.labels
      taints            = v.taint
    }
  }
}

output "node_group_arns" {
  description = "List of the EKS node group ARNs"
  value       = [for ng in aws_eks_node_group.this : ng.arn]
}

output "node_group_statuses" {
  description = "Status of the EKS node groups"
  value       = { for k, v in aws_eks_node_group.this : k => v.status }
}

# IAM Roles
output "cluster_iam_role_name" {
  description = "IAM role name associated with EKS cluster"
  value       = aws_iam_role.cluster.name
}

output "cluster_iam_role_arn" {
  description = "IAM role ARN associated with EKS cluster"
  value       = aws_iam_role.cluster.arn
}

output "node_group_iam_role_name" {
  description = "IAM role name associated with EKS node group"
  value       = aws_iam_role.node_group.name
}

output "node_group_iam_role_arn" {
  description = "IAM role ARN associated with EKS node group"
  value       = aws_iam_role.node_group.arn
}

# Service Account IAM Roles
output "aws_load_balancer_controller_role_arn" {
  description = "IAM role ARN for AWS Load Balancer Controller"
  value       = var.enable_aws_load_balancer_controller ? aws_iam_role.aws_load_balancer_controller[0].arn : null
}

output "ebs_csi_driver_role_arn" {
  description = "IAM role ARN for EBS CSI driver"
  value       = var.enable_ebs_csi_driver ? aws_iam_role.ebs_csi_driver[0].arn : null
}

# Add-ons
output "cluster_addons" {
  description = "Map of attribute maps for all EKS cluster addons enabled"
  value = {
    for k, v in aws_eks_addon.this : k => {
      arn               = v.arn
      status            = v.status
      addon_version     = v.addon_version
      resolve_conflicts = v.resolve_conflicts
    }
  }
}

# Encryption
output "kms_key_id" {
  description = "The globally unique identifier for the KMS key used for envelope encryption"
  value       = var.cluster_encryption_config_enabled ? aws_kms_key.eks.key_id : null
}

output "kms_key_arn" {
  description = "The Amazon Resource Name (ARN) of the KMS key used for envelope encryption"
  value       = var.cluster_encryption_config_enabled ? aws_kms_key.eks.arn : null
}

# CloudWatch
output "cloudwatch_log_group_name" {
  description = "Name of cloudwatch log group created"
  value       = var.create_cloudwatch_log_group ? aws_cloudwatch_log_group.cluster.name : null
}

output "cloudwatch_log_group_arn" {
  description = "Arn of cloudwatch log group created"
  value       = var.create_cloudwatch_log_group ? aws_cloudwatch_log_group.cluster.arn : null
}

# Kubeconfig
output "kubeconfig" {
  description = "kubectl config as generated by the module"
  value = {
    apiVersion      = "v1"
    kind            = "Config"
    current_context = "terraform"
    contexts = [{
      name = "terraform"
      context = {
        cluster = "terraform"
        user    = "terraform"
      }
    }]
    clusters = [{
      name = "terraform"
      cluster = {
        certificate_authority_data = aws_eks_cluster.this.certificate_authority[0].data
        server                     = aws_eks_cluster.this.endpoint
      }
    }]
    users = [{
      name = "terraform"
      user = {
        exec = {
          apiVersion = "client.authentication.k8s.io/v1beta1"
          command    = "aws"
          args = [
            "eks",
            "get-token",
            "--cluster-name",
            aws_eks_cluster.this.name,
          ]
        }
      }
    }]
  }
}

# Data source for cluster authentication
data "aws_eks_cluster_auth" "cluster" {
  name = aws_eks_cluster.this.name
}