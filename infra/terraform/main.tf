# Bunkerverse Platform - Infrastructure as Code
# Complete cloud infrastructure for secure, scalable deployment
# Version: 1.0.0

terraform {
  required_version = ">= 1.0"
  
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.20"
    }
    helm = {
      source  = "hashicorp/helm"
      version = "~> 2.10"
    }
  }

  backend "s3" {
    # Configure remote state storage
    # bucket = "bunkerverse-terraform-state"
    # key    = "infrastructure/terraform.tfstate"
    # region = "us-east-1"
    # encrypt = true
    # dynamodb_table = "bunkerverse-terraform-locks"
  }
}

# Provider Configuration
provider "aws" {
  region = var.aws_region
  
  default_tags {
    tags = {
      Project     = "Bunkerverse"
      Environment = var.environment
      ManagedBy   = "Terraform"
      Owner       = "Platform-Team"
      CostCenter  = "Engineering"
    }
  }
}

# Data sources
data "aws_caller_identity" "current" {}
data "aws_availability_zones" "available" {
  state = "available"
}

# Local values for consistent naming
locals {
  name_prefix = "${var.project_name}-${var.environment}"
  
  common_tags = {
    Project     = var.project_name
    Environment = var.environment
    ManagedBy   = "Terraform"
    Owner       = "Platform-Team"
  }

  # Network configuration
  vpc_cidr = var.vpc_cidr
  azs = slice(data.aws_availability_zones.available.names, 0, 3)
}

# Networking Module
module "networking" {
  source = "./modules/networking"
  
  name_prefix = local.name_prefix
  vpc_cidr    = local.vpc_cidr
  azs         = local.azs
  
  # Public subnets for load balancers
  public_subnet_cidrs = [
    cidrsubnet(local.vpc_cidr, 8, 1),
    cidrsubnet(local.vpc_cidr, 8, 2),
    cidrsubnet(local.vpc_cidr, 8, 3)
  ]
  
  # Private subnets for EKS nodes
  private_subnet_cidrs = [
    cidrsubnet(local.vpc_cidr, 8, 10),
    cidrsubnet(local.vpc_cidr, 8, 11),
    cidrsubnet(local.vpc_cidr, 8, 12)
  ]
  
  # Database subnets
  database_subnet_cidrs = [
    cidrsubnet(local.vpc_cidr, 8, 20),
    cidrsubnet(local.vpc_cidr, 8, 21),
    cidrsubnet(local.vpc_cidr, 8, 22)
  ]
  
  enable_nat_gateway = true
  enable_vpn_gateway = false
  enable_dns_hostnames = true
  enable_dns_support = true
  
  tags = local.common_tags
}

# Container Registry Module
module "container_registry" {
  source = "./modules/container_registry"
  
  name_prefix = local.name_prefix
  
  # Repositories for each service
  repositories = [
    "marketplace-service",
    "indexer-service", 
    "identity-service",
    "ai-data-service",
    "account-service",
    "feedback-service",
    "mission-service",
    "payment-service",
    "social-service"
  ]
  
  # Lifecycle policy for cost optimization
  lifecycle_policy_rules = [
    {
      rulePriority = 1
      description  = "Keep last 10 production images"
      selection = {
        tagStatus     = "tagged"
        tagPrefixList = ["v", "release"]
        countType     = "imageCountMoreThan"
        countNumber   = 10
      }
      action = {
        type = "expire"
      }
    },
    {
      rulePriority = 2
      description  = "Delete untagged images after 1 day"
      selection = {
        tagStatus   = "untagged"
        countType   = "sinceImagePushed"
        countUnit   = "days"
        countNumber = 1
      }
      action = {
        type = "expire"
      }
    }
  ]
  
  tags = local.common_tags
}

# EKS Cluster Module
module "eks_cluster" {
  source = "./modules/eks_cluster"
  
  name_prefix = local.name_prefix
  
  # Network configuration
  vpc_id                   = module.networking.vpc_id
  private_subnet_ids       = module.networking.private_subnet_ids
  public_subnet_ids        = module.networking.public_subnet_ids
  
  # Cluster configuration
  cluster_version          = "1.28"
  cluster_endpoint_private = false
  cluster_endpoint_public  = true
  
  # Node group configuration
  node_groups = {
    main = {
      instance_types = ["t3.medium", "t3.large"]
      capacity_type  = "ON_DEMAND"
      
      scaling_config = {
        desired_size = 3
        max_size     = 10
        min_size     = 1
      }
      
      update_config = {
        max_unavailable_percentage = 25
      }
      
      labels = {
        role = "general"
      }
    }
    
    game_servers = {
      instance_types = ["c5.large", "c5.xlarge"]
      capacity_type  = "SPOT"
      
      scaling_config = {
        desired_size = 0
        max_size     = 20
        min_size     = 0
      }
      
      update_config = {
        max_unavailable_percentage = 50
      }
      
      labels = {
        role = "game-servers"
      }
      
      taints = [
        {
          key    = "dedicated"
          value  = "game-servers"
          effect = "NO_SCHEDULE"
        }
      ]
    }
  }
  
  # Add-ons
  cluster_addons = {
    coredns = {
      resolve_conflicts = "OVERWRITE"
    }
    kube-proxy = {
      resolve_conflicts = "OVERWRITE"
    }
    vpc-cni = {
      resolve_conflicts = "OVERWRITE"
    }
    aws-ebs-csi-driver = {
      resolve_conflicts = "OVERWRITE"
    }
  }
  
  tags = local.common_tags
}

# Database Module
module "databases" {
  source = "./modules/databases"
  
  name_prefix = local.name_prefix
  
  # Network configuration
  vpc_id               = module.networking.vpc_id
  database_subnet_ids  = module.networking.database_subnet_ids
  
  # RDS Configuration
  rds_config = {
    engine         = "mysql"
    engine_version = "8.0"
    instance_class = var.environment == "prod" ? "db.r5.large" : "db.t3.micro"
    
    allocated_storage     = var.environment == "prod" ? 100 : 20
    max_allocated_storage = var.environment == "prod" ? 1000 : 100
    
    database_name = "bunkerverse"
    username      = "admin"
    
    backup_retention_period = var.environment == "prod" ? 7 : 1
    backup_window          = "03:00-04:00"
    maintenance_window     = "sun:04:00-sun:05:00"
    
    deletion_protection = var.environment == "prod"
    skip_final_snapshot = var.environment != "prod"
    
    performance_insights_enabled = var.environment == "prod"
    monitoring_interval         = var.environment == "prod" ? 60 : 0
    
    enabled_cloudwatch_logs_exports = ["error", "general", "slowquery"]
  }
  
  # ElastiCache Redis Configuration
  redis_config = {
    node_type               = var.environment == "prod" ? "cache.r5.large" : "cache.t3.micro"
    num_cache_clusters      = var.environment == "prod" ? 2 : 1
    parameter_group_name    = "default.redis7"
    port                   = 6379
    
    # Backup configuration
    snapshot_retention_limit = var.environment == "prod" ? 7 : 1
    snapshot_window         = "03:00-05:00"
    
    # Maintenance
    maintenance_window = "sun:05:00-sun:07:00"
    
    # Security
    at_rest_encryption_enabled = true
    transit_encryption_enabled = true
    auth_token_enabled         = true
  }
  
  # OpenSearch Configuration
  opensearch_config = {
    engine_version = "OpenSearch_2.3"
    instance_type  = var.environment == "prod" ? "r6g.large.search" : "t3.small.search"
    instance_count = var.environment == "prod" ? 3 : 1
    
    dedicated_master_enabled = var.environment == "prod"
    master_instance_type     = var.environment == "prod" ? "r6g.medium.search" : null
    master_instance_count    = var.environment == "prod" ? 3 : 0
    
    ebs_enabled = true
    volume_type = "gp3"
    volume_size = var.environment == "prod" ? 100 : 20
    
    encrypt_at_rest = {
      enabled = true
    }
    
    node_to_node_encryption = {
      enabled = true
    }
    
    domain_endpoint_options = {
      enforce_https       = true
      tls_security_policy = "Policy-Min-TLS-1-2-2019-07"
    }
  }
  
  tags = local.common_tags
}

# Secrets Management
module "secrets" {
  source = "./modules/secrets"
  
  name_prefix = local.name_prefix
  
  # Database secrets
  database_password = module.databases.rds_password
  redis_auth_token  = module.databases.redis_auth_token
  
  # Application secrets
  jwt_secret = var.jwt_secret
  
  tags = local.common_tags
}

# Load Balancer and Ingress
module "ingress" {
  source = "./modules/ingress"
  
  name_prefix = local.name_prefix
  
  # Network configuration
  vpc_id            = module.networking.vpc_id
  public_subnet_ids = module.networking.public_subnet_ids
  
  # EKS cluster
  cluster_name                = module.eks_cluster.cluster_name
  cluster_endpoint            = module.eks_cluster.cluster_endpoint
  cluster_certificate_authority_data = module.eks_cluster.cluster_certificate_authority_data
  
  # SSL Certificate
  certificate_domain = var.domain_name
  
  tags = local.common_tags
}

# Agones Game Server Management
module "agones" {
  source = "./modules/agones"
  
  depends_on = [module.eks_cluster]
  
  # EKS cluster configuration
  cluster_name                        = module.eks_cluster.cluster_name
  cluster_endpoint                    = module.eks_cluster.cluster_endpoint
  cluster_certificate_authority_data = module.eks_cluster.cluster_certificate_authority_data
  
  # Agones configuration
  agones_version = "1.36.0"
  
  # Game server fleet configuration
  gameserver_config = {
    replicas = var.environment == "prod" ? 5 : 1
    
    template = {
      spec = {
        ports = [
          {
            name          = "default"
            portPolicy    = "Dynamic"
            containerPort = 7654
            protocol      = "UDP"
          }
        ]
        
        health = {
          disabled               = false
          initialDelaySeconds    = 5
          periodSeconds         = 5
          failureThreshold      = 3
        }
      }
    }
  }
  
  tags = local.common_tags
}