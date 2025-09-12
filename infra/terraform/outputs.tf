# Bunkerverse Platform - Terraform Outputs
# Infrastructure resource information and connection details

#################################################
# Network Outputs
#################################################

output "vpc_id" {
  description = "VPC ID"
  value       = module.networking.vpc_id
}

output "vpc_cidr_block" {
  description = "VPC CIDR block"
  value       = module.networking.vpc_cidr_block
}

output "public_subnet_ids" {
  description = "Public subnet IDs"
  value       = module.networking.public_subnet_ids
}

output "private_subnet_ids" {
  description = "Private subnet IDs"
  value       = module.networking.private_subnet_ids
}

output "database_subnet_ids" {
  description = "Database subnet IDs"
  value       = module.networking.database_subnet_ids
}

output "nat_gateway_ips" {
  description = "NAT Gateway public IP addresses"
  value       = module.networking.nat_gateway_ips
}

#################################################
# EKS Cluster Outputs
#################################################

output "cluster_name" {
  description = "EKS cluster name"
  value       = module.eks_cluster.cluster_name
}

output "cluster_endpoint" {
  description = "EKS cluster endpoint"
  value       = module.eks_cluster.cluster_endpoint
}

output "cluster_version" {
  description = "EKS cluster Kubernetes version"
  value       = module.eks_cluster.cluster_version
}

output "cluster_security_group_id" {
  description = "EKS cluster security group ID"
  value       = module.eks_cluster.cluster_security_group_id
}

output "cluster_oidc_issuer_url" {
  description = "EKS cluster OIDC issuer URL"
  value       = module.eks_cluster.cluster_oidc_issuer_url
}

output "node_group_arns" {
  description = "EKS node group ARNs"
  value       = module.eks_cluster.node_group_arns
}

#################################################
# Container Registry Outputs
#################################################

output "ecr_repository_urls" {
  description = "ECR repository URLs"
  value       = module.container_registry.repository_urls
}

output "ecr_repository_arns" {
  description = "ECR repository ARNs"
  value       = module.container_registry.repository_arns
}

#################################################
# Database Outputs
#################################################

output "rds_endpoint" {
  description = "RDS instance endpoint"
  value       = module.databases.rds_endpoint
  sensitive   = false
}

output "rds_port" {
  description = "RDS instance port"
  value       = module.databases.rds_port
}

output "rds_database_name" {
  description = "RDS database name"
  value       = module.databases.rds_database_name
}

output "rds_username" {
  description = "RDS master username"
  value       = module.databases.rds_username
  sensitive   = true
}

output "redis_endpoint" {
  description = "ElastiCache Redis endpoint"
  value       = module.databases.redis_endpoint
}

output "redis_port" {
  description = "ElastiCache Redis port"
  value       = module.databases.redis_port
}

output "opensearch_endpoint" {
  description = "OpenSearch domain endpoint"
  value       = module.databases.opensearch_endpoint
}

output "opensearch_dashboard_endpoint" {
  description = "OpenSearch dashboard endpoint"
  value       = module.databases.opensearch_dashboard_endpoint
}

#################################################
# Secrets Management Outputs
#################################################

output "secrets_manager_arns" {
  description = "AWS Secrets Manager secret ARNs"
  value       = module.secrets.secret_arns
  sensitive   = true
}

output "database_secret_arn" {
  description = "Database password secret ARN"
  value       = module.secrets.database_secret_arn
  sensitive   = true
}

output "redis_secret_arn" {
  description = "Redis auth token secret ARN"
  value       = module.secrets.redis_secret_arn
  sensitive   = true
}

output "jwt_secret_arn" {
  description = "JWT secret ARN"
  value       = module.secrets.jwt_secret_arn
  sensitive   = true
}

#################################################
# Load Balancer and Ingress Outputs
#################################################

output "alb_dns_name" {
  description = "Application Load Balancer DNS name"
  value       = module.ingress.alb_dns_name
}

output "alb_zone_id" {
  description = "Application Load Balancer hosted zone ID"
  value       = module.ingress.alb_zone_id
}

output "ingress_controller_service_account_role_arn" {
  description = "ALB Ingress Controller service account role ARN"
  value       = module.ingress.ingress_controller_role_arn
}

output "certificate_arn" {
  description = "SSL certificate ARN"
  value       = module.ingress.certificate_arn
}

#################################################
# Agones Game Server Outputs
#################################################

output "agones_namespace" {
  description = "Agones system namespace"
  value       = module.agones.agones_namespace
}

output "gameserver_fleet_name" {
  description = "Game server fleet name"
  value       = module.agones.gameserver_fleet_name
}

output "agones_controller_service_account_role_arn" {
  description = "Agones controller service account role ARN"
  value       = module.agones.agones_controller_role_arn
}

#################################################
# Connection Information
#################################################

output "kubectl_config_command" {
  description = "Command to configure kubectl for EKS cluster"
  value       = "aws eks update-kubeconfig --region ${var.aws_region} --name ${module.eks_cluster.cluster_name}"
}

output "database_connection_string" {
  description = "Database connection string template"
  value       = "postgresql://${module.databases.rds_username}:[PASSWORD]@${module.databases.rds_endpoint}:${module.databases.rds_port}/${module.databases.rds_database_name}"
  sensitive   = true
}

output "redis_connection_string" {
  description = "Redis connection string template"
  value       = "redis://[AUTH_TOKEN]@${module.databases.redis_endpoint}:${module.databases.redis_port}"
  sensitive   = true
}

output "opensearch_connection_string" {
  description = "OpenSearch connection string"
  value       = "https://${module.databases.opensearch_endpoint}"
}

#################################################
# Monitoring and Logging
#################################################

output "cloudwatch_log_groups" {
  description = "CloudWatch log group names"
  value = {
    eks_cluster = "/aws/eks/${module.eks_cluster.cluster_name}/cluster"
    vpc_flow    = "/aws/vpc/flowlogs"
  }
}

output "monitoring_dashboard_url" {
  description = "CloudWatch dashboard URL"
  value       = "https://${var.aws_region}.console.aws.amazon.com/cloudwatch/home?region=${var.aws_region}#dashboards:name=${local.name_prefix}-monitoring"
}

#################################################
# Cost and Resource Information
#################################################

output "resource_summary" {
  description = "Summary of deployed resources"
  value = {
    vpc_id           = module.networking.vpc_id
    cluster_name     = module.eks_cluster.cluster_name
    rds_instance_id  = module.databases.rds_instance_id
    redis_cluster_id = module.databases.redis_cluster_id
    opensearch_domain = module.databases.opensearch_domain_name
    
    ecr_repositories = length(module.container_registry.repository_urls)
    public_subnets   = length(module.networking.public_subnet_ids)
    private_subnets  = length(module.networking.private_subnet_ids)
    database_subnets = length(module.networking.database_subnet_ids)
  }
}

output "estimated_monthly_cost" {
  description = "Estimated monthly cost breakdown (USD)"
  value = {
    message = "Cost estimates are approximate and may vary based on usage patterns"
    
    compute = {
      eks_cluster = "Free tier: $0.10/hour for cluster management"
      node_groups = "t3.medium: ~$30-45/month per node (based on usage)"
    }
    
    storage = {
      rds         = "db.t3.micro: ~$15-20/month + storage costs"
      redis       = "cache.t3.micro: ~$15-20/month"
      opensearch  = "t3.small.search: ~$25-30/month"
    }
    
    networking = {
      nat_gateway = "~$45/month per NAT Gateway"
      data_transfer = "Variable based on traffic"
    }
    
    total_estimate = var.environment == "prod" ? "$200-400/month" : "$100-200/month"
  }
}