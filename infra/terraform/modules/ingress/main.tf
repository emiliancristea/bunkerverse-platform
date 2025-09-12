# Ingress Module
# Creates ALB with SSL termination, AWS Load Balancer Controller, and Certificate Manager integration

terraform {
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
}

# Data sources
data "aws_caller_identity" "current" {}
data "aws_partition" "current" {}
data "aws_region" "current" {}

# Configure Kubernetes provider
data "aws_eks_cluster" "cluster" {
  name = var.cluster_name
}

data "aws_eks_cluster_auth" "cluster" {
  name = var.cluster_name
}

provider "kubernetes" {
  host                   = data.aws_eks_cluster.cluster.endpoint
  cluster_ca_certificate = base64decode(data.aws_eks_cluster.cluster.certificate_authority.0.data)
  token                  = data.aws_eks_cluster_auth.cluster.token
}

provider "helm" {
  kubernetes {
    host                   = data.aws_eks_cluster.cluster.endpoint
    cluster_ca_certificate = base64decode(data.aws_eks_cluster.cluster.certificate_authority.0.data)
    token                  = data.aws_eks_cluster_auth.cluster.token
  }
}

#################################################
# Certificate Management
#################################################

# Route53 Hosted Zone (if domain validation is used)
data "aws_route53_zone" "main" {
  count = var.certificate_domain != "" && var.certificate_validation_method == "DNS" ? 1 : 0
  name  = var.certificate_domain
}

# ACM Certificate
resource "aws_acm_certificate" "main" {
  count             = var.certificate_domain != "" ? 1 : 0
  domain_name       = var.certificate_domain
  validation_method = var.certificate_validation_method

  subject_alternative_names = var.certificate_subject_alternative_names

  lifecycle {
    create_before_destroy = true
  }

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-ssl-certificate"
  })
}

# Certificate validation (DNS method)
resource "aws_route53_record" "certificate_validation" {
  for_each = var.certificate_domain != "" && var.certificate_validation_method == "DNS" ? {
    for dvo in aws_acm_certificate.main[0].domain_validation_options : dvo.domain_name => {
      name   = dvo.resource_record_name
      record = dvo.resource_record_value
      type   = dvo.resource_record_type
    }
  } : {}

  allow_overwrite = true
  name            = each.value.name
  records         = [each.value.record]
  ttl             = 60
  type            = each.value.type
  zone_id         = data.aws_route53_zone.main[0].zone_id
}

resource "aws_acm_certificate_validation" "main" {
  count           = var.certificate_domain != "" && var.certificate_validation_method == "DNS" ? 1 : 0
  certificate_arn = aws_acm_certificate.main[0].arn
  validation_record_fqdns = [for record in aws_route53_record.certificate_validation : record.fqdn]

  timeouts {
    create = "5m"
  }
}

#################################################
# Security Groups
#################################################

# Security Group for ALB
resource "aws_security_group" "alb" {
  name_prefix = "${var.name_prefix}-alb-"
  vpc_id      = var.vpc_id
  description = "Security group for Application Load Balancer"

  # HTTP access from anywhere
  ingress {
    description = "HTTP"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = var.allowed_cidr_blocks
  }

  # HTTPS access from anywhere
  ingress {
    description = "HTTPS"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = var.allowed_cidr_blocks
  }

  # Allow all outbound traffic
  egress {
    description = "All outbound traffic"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-alb-sg"
  })

  lifecycle {
    create_before_destroy = true
  }
}

# Additional security group rules
resource "aws_security_group_rule" "alb_additional" {
  for_each = var.additional_security_group_rules

  type              = each.value.type
  from_port         = each.value.from_port
  to_port           = each.value.to_port
  protocol          = each.value.protocol
  cidr_blocks       = lookup(each.value, "cidr_blocks", null)
  security_group_id = aws_security_group.alb.id
  description       = each.value.description
}

#################################################
# IAM Role for AWS Load Balancer Controller
#################################################

# OIDC provider data
data "aws_iam_openid_connect_provider" "eks" {
  url = var.cluster_oidc_issuer_url
}

# IAM role for AWS Load Balancer Controller
resource "aws_iam_role" "aws_load_balancer_controller" {
  name = "${var.name_prefix}-aws-load-balancer-controller"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Federated = data.aws_iam_openid_connect_provider.eks.arn
        }
        Action = "sts:AssumeRoleWithWebIdentity"
        Condition = {
          StringEquals = {
            "${replace(data.aws_iam_openid_connect_provider.eks.url, "https://", "")}:sub": "system:serviceaccount:kube-system:aws-load-balancer-controller"
            "${replace(data.aws_iam_openid_connect_provider.eks.url, "https://", "")}:aud": "sts.amazonaws.com"
          }
        }
      }
    ]
  })

  tags = var.tags
}

# IAM policy for AWS Load Balancer Controller
resource "aws_iam_role_policy" "aws_load_balancer_controller" {
  name = "${var.name_prefix}-aws-load-balancer-controller"
  role = aws_iam_role.aws_load_balancer_controller.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "iam:CreateServiceLinkedRole"
        ]
        Resource = "*"
        Condition = {
          StringEquals = {
            "iam:AWSServiceName" = "elasticloadbalancing.amazonaws.com"
          }
        }
      },
      {
        Effect = "Allow"
        Action = [
          "ec2:DescribeAccountAttributes",
          "ec2:DescribeAddresses",
          "ec2:DescribeAvailabilityZones",
          "ec2:DescribeInternetGateways",
          "ec2:DescribeVpcs",
          "ec2:DescribeVpcPeeringConnections",
          "ec2:DescribeSubnets",
          "ec2:DescribeSecurityGroups",
          "ec2:DescribeInstances",
          "ec2:DescribeNetworkInterfaces",
          "ec2:DescribeTags",
          "ec2:GetCoipPoolUsage",
          "ec2:DescribeCoipPools",
          "elasticloadbalancing:DescribeLoadBalancers",
          "elasticloadbalancing:DescribeLoadBalancerAttributes",
          "elasticloadbalancing:DescribeListeners",
          "elasticloadbalancing:DescribeListenerCertificates",
          "elasticloadbalancing:DescribeSSLPolicies",
          "elasticloadbalancing:DescribeRules",
          "elasticloadbalancing:DescribeTargetGroups",
          "elasticloadbalancing:DescribeTargetGroupAttributes",
          "elasticloadbalancing:DescribeTargetHealth",
          "elasticloadbalancing:DescribeTags"
        ]
        Resource = "*"
      },
      {
        Effect = "Allow"
        Action = [
          "cognito-idp:DescribeUserPoolClient",
          "acm:ListCertificates",
          "acm:DescribeCertificate",
          "iam:ListServerCertificates",
          "iam:GetServerCertificate",
          "waf-regional:GetWebACL",
          "waf-regional:GetWebACLForResource",
          "waf-regional:AssociateWebACL",
          "waf-regional:DisassociateWebACL",
          "wafv2:GetWebACL",
          "wafv2:GetWebACLForResource",
          "wafv2:AssociateWebACL",
          "wafv2:DisassociateWebACL",
          "shield:DescribeProtection",
          "shield:GetSubscriptionState",
          "shield:DescribeSubscription",
          "shield:CreateProtection",
          "shield:DeleteProtection"
        ]
        Resource = "*"
      },
      {
        Effect = "Allow"
        Action = [
          "ec2:AuthorizeSecurityGroupIngress",
          "ec2:RevokeSecurityGroupIngress"
        ]
        Resource = "*"
      },
      {
        Effect = "Allow"
        Action = [
          "ec2:CreateSecurityGroup"
        ]
        Resource = "*"
      },
      {
        Effect = "Allow"
        Action = [
          "ec2:CreateTags"
        ]
        Resource = "arn:${data.aws_partition.current.partition}:ec2:*:*:security-group/*"
        Condition = {
          StringEquals = {
            "ec2:CreateAction" = "CreateSecurityGroup"
          }
          Null = {
            "aws:RequestTag/elbv2.k8s.aws/cluster" = "false"
          }
        }
      },
      {
        Effect = "Allow"
        Action = [
          "elasticloadbalancing:CreateLoadBalancer",
          "elasticloadbalancing:CreateTargetGroup"
        ]
        Resource = "*"
        Condition = {
          Null = {
            "aws:RequestTag/elbv2.k8s.aws/cluster" = "false"
          }
        }
      },
      {
        Effect = "Allow"
        Action = [
          "elasticloadbalancing:CreateListener",
          "elasticloadbalancing:DeleteListener",
          "elasticloadbalancing:CreateRule",
          "elasticloadbalancing:DeleteRule"
        ]
        Resource = "*"
      },
      {
        Effect = "Allow"
        Action = [
          "elasticloadbalancing:AddTags",
          "elasticloadbalancing:RemoveTags"
        ]
        Resource = [
          "arn:${data.aws_partition.current.partition}:elasticloadbalancing:*:*:targetgroup/*/*",
          "arn:${data.aws_partition.current.partition}:elasticloadbalancing:*:*:loadbalancer/net/*/*",
          "arn:${data.aws_partition.current.partition}:elasticloadbalancing:*:*:loadbalancer/app/*/*"
        ]
        Condition = {
          Null = {
            "aws:RequestTag/elbv2.k8s.aws/cluster" = "true"
            "aws:ResourceTag/elbv2.k8s.aws/cluster" = "false"
          }
        }
      },
      {
        Effect = "Allow"
        Action = [
          "elasticloadbalancing:RegisterTargets",
          "elasticloadbalancing:DeregisterTargets"
        ]
        Resource = "arn:${data.aws_partition.current.partition}:elasticloadbalancing:*:*:targetgroup/*/*"
      },
      {
        Effect = "Allow"
        Action = [
          "elasticloadbalancing:SetWebAcl",
          "elasticloadbalancing:ModifyListener",
          "elasticloadbalancing:AddListenerCertificates",
          "elasticloadbalancing:RemoveListenerCertificates",
          "elasticloadbalancing:ModifyRule"
        ]
        Resource = "*"
      }
    ]
  })
}

#################################################
# Kubernetes Resources
#################################################

# Namespace for ingress resources
resource "kubernetes_namespace" "ingress" {
  count = var.create_namespace ? 1 : 0
  
  metadata {
    name = var.ingress_namespace
    labels = {
      name                               = var.ingress_namespace
      "app.kubernetes.io/name"          = "ingress-nginx"
      "app.kubernetes.io/instance"      = var.name_prefix
      "app.kubernetes.io/managed-by"    = "terraform"
    }
  }
}

# Service account for AWS Load Balancer Controller
resource "kubernetes_service_account" "aws_load_balancer_controller" {
  metadata {
    name      = "aws-load-balancer-controller"
    namespace = "kube-system"
    labels = {
      "app.kubernetes.io/component" = "controller"
      "app.kubernetes.io/name"      = "aws-load-balancer-controller"
      "app.kubernetes.io/managed-by" = "terraform"
    }
    annotations = {
      "eks.amazonaws.com/role-arn"               = aws_iam_role.aws_load_balancer_controller.arn
      "serviceaccount.kubernetes.io/aws-role-arn" = aws_iam_role.aws_load_balancer_controller.arn
    }
  }

  automount_service_account_token = true
}

#################################################
# Helm Chart - AWS Load Balancer Controller
#################################################

resource "helm_release" "aws_load_balancer_controller" {
  name       = "aws-load-balancer-controller"
  repository = "https://aws.github.io/eks-charts"
  chart      = "aws-load-balancer-controller"
  namespace  = "kube-system"
  version    = var.aws_load_balancer_controller_chart_version

  set {
    name  = "clusterName"
    value = var.cluster_name
  }

  set {
    name  = "serviceAccount.create"
    value = "false"
  }

  set {
    name  = "serviceAccount.name"
    value = kubernetes_service_account.aws_load_balancer_controller.metadata[0].name
  }

  set {
    name  = "region"
    value = data.aws_region.current.name
  }

  set {
    name  = "vpcId"
    value = var.vpc_id
  }

  set {
    name  = "image.repository"
    value = var.aws_load_balancer_controller_image_repository
  }

  set {
    name  = "image.tag"
    value = var.aws_load_balancer_controller_image_tag
  }

  # Resource limits and requests
  set {
    name  = "resources.limits.cpu"
    value = "200m"
  }

  set {
    name  = "resources.limits.memory"
    value = "500Mi"
  }

  set {
    name  = "resources.requests.cpu"
    value = "100m"
  }

  set {
    name  = "resources.requests.memory"
    value = "200Mi"
  }

  # Node selector and tolerations
  dynamic "set" {
    for_each = var.node_selector
    content {
      name  = "nodeSelector.${set.key}"
      value = set.value
    }
  }

  dynamic "set" {
    for_each = var.tolerations
    content {
      name  = "tolerations[${set.key}].key"
      value = set.value.key
    }
  }

  dynamic "set" {
    for_each = var.tolerations
    content {
      name  = "tolerations[${set.key}].operator"
      value = set.value.operator
    }
  }

  dynamic "set" {
    for_each = var.tolerations
    content {
      name  = "tolerations[${set.key}].value"
      value = lookup(set.value, "value", "")
    }
  }

  dynamic "set" {
    for_each = var.tolerations
    content {
      name  = "tolerations[${set.key}].effect"
      value = set.value.effect
    }
  }

  # Additional configuration
  set {
    name  = "replicaCount"
    value = var.controller_replica_count
  }

  set {
    name  = "enableServiceMutatorWebhook"
    value = "false"
  }

  set {
    name  = "enableCertManager"
    value = var.enable_cert_manager_integration
  }

  depends_on = [kubernetes_service_account.aws_load_balancer_controller]
}

#################################################
# Ingress Class
#################################################

resource "kubernetes_ingress_class_v1" "alb" {
  metadata {
    name = var.ingress_class_name
    labels = {
      "app.kubernetes.io/name"       = "aws-load-balancer-controller"
      "app.kubernetes.io/managed-by" = "terraform"
    }
  }

  spec {
    controller = "ingress.k8s.aws/alb"
    
    parameters {
      api_group = "elbv2.k8s.aws"
      kind      = "IngressClassParams"
      name      = kubernetes_manifest.ingress_class_params.manifest.metadata.name
    }
  }

  depends_on = [helm_release.aws_load_balancer_controller]
}

# Ingress class parameters
resource "kubernetes_manifest" "ingress_class_params" {
  manifest = {
    apiVersion = "elbv2.k8s.aws/v1beta1"
    kind       = "IngressClassParams"
    metadata = {
      name = "${var.name_prefix}-alb-params"
      namespace = "kube-system"
    }
    spec = {
      scheme = "internet-facing"
      ipAddressType = "ipv4"
      
      tags = merge(var.tags, {
        "kubernetes.io/cluster/${var.cluster_name}" = "owned"
        "kubernetes.io/ingress-name" = var.name_prefix
      })

      group = {
        name = var.name_prefix
      }

      loadBalancerAttributes = [
        {
          key   = "idle_timeout.timeout_seconds"
          value = tostring(var.load_balancer_idle_timeout)
        },
        {
          key   = "routing.http2.enabled"
          value = tostring(var.enable_http2)
        },
        {
          key   = "access_logs.s3.enabled"
          value = tostring(var.enable_access_logs)
        }
      ]

      # Access logs configuration
      dynamic "loadBalancerAttributes" {
        for_each = var.enable_access_logs && var.access_logs_s3_bucket != "" ? [1] : []
        content {
          key   = "access_logs.s3.bucket"
          value = var.access_logs_s3_bucket
        }
      }

      dynamic "loadBalancerAttributes" {
        for_each = var.enable_access_logs && var.access_logs_s3_prefix != "" ? [1] : []
        content {
          key   = "access_logs.s3.prefix"
          value = var.access_logs_s3_prefix
        }
      }

      # WAF integration
      dynamic "wafv2" {
        for_each = var.waf_acl_arn != "" ? [1] : []
        content {
          webACLARN = var.waf_acl_arn
        }
      }
    }
  }

  depends_on = [helm_release.aws_load_balancer_controller]
}

#################################################
# Default Backend Service (Optional)
#################################################

resource "kubernetes_deployment" "default_backend" {
  count = var.create_default_backend ? 1 : 0

  metadata {
    name      = "${var.name_prefix}-default-backend"
    namespace = var.create_namespace ? kubernetes_namespace.ingress[0].metadata[0].name : var.ingress_namespace
    labels = {
      "app.kubernetes.io/name"       = "default-backend"
      "app.kubernetes.io/instance"   = var.name_prefix
      "app.kubernetes.io/managed-by" = "terraform"
    }
  }

  spec {
    replicas = 2

    selector {
      match_labels = {
        "app.kubernetes.io/name"     = "default-backend"
        "app.kubernetes.io/instance" = var.name_prefix
      }
    }

    template {
      metadata {
        labels = {
          "app.kubernetes.io/name"     = "default-backend"
          "app.kubernetes.io/instance" = var.name_prefix
        }
      }

      spec {
        container {
          name  = "default-backend"
          image = "registry.k8s.io/defaultbackend-amd64:1.5"
          port {
            container_port = 8080
          }

          resources {
            limits = {
              cpu    = "10m"
              memory = "20Mi"
            }
            requests = {
              cpu    = "10m"
              memory = "20Mi"
            }
          }

          liveness_probe {
            http_get {
              path = "/healthz"
              port = 8080
            }
            initial_delay_seconds = 30
            timeout_seconds       = 5
          }

          readiness_probe {
            http_get {
              path = "/healthz"
              port = 8080
            }
            initial_delay_seconds = 0
            timeout_seconds       = 5
          }
        }
      }
    }
  }
}

resource "kubernetes_service" "default_backend" {
  count = var.create_default_backend ? 1 : 0

  metadata {
    name      = "${var.name_prefix}-default-backend"
    namespace = var.create_namespace ? kubernetes_namespace.ingress[0].metadata[0].name : var.ingress_namespace
    labels = {
      "app.kubernetes.io/name"       = "default-backend"
      "app.kubernetes.io/instance"   = var.name_prefix
      "app.kubernetes.io/managed-by" = "terraform"
    }
  }

  spec {
    selector = {
      "app.kubernetes.io/name"     = "default-backend"
      "app.kubernetes.io/instance" = var.name_prefix
    }

    port {
      port        = 80
      target_port = 8080
    }

    type = "ClusterIP"
  }
}

#################################################
# CloudWatch Logging for Load Balancer
#################################################

resource "aws_s3_bucket" "access_logs" {
  count  = var.enable_access_logs && var.access_logs_s3_bucket == "" ? 1 : 0
  bucket = "${var.name_prefix}-alb-access-logs-${random_string.bucket_suffix[0].result}"

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-alb-access-logs"
  })
}

resource "random_string" "bucket_suffix" {
  count   = var.enable_access_logs && var.access_logs_s3_bucket == "" ? 1 : 0
  length  = 8
  special = false
  upper   = false
}

resource "aws_s3_bucket_policy" "access_logs" {
  count  = var.enable_access_logs && var.access_logs_s3_bucket == "" ? 1 : 0
  bucket = aws_s3_bucket.access_logs[0].id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          AWS = "arn:aws:iam::${local.elb_service_account_mapping[data.aws_region.current.name]}:root"
        }
        Action   = "s3:PutObject"
        Resource = "${aws_s3_bucket.access_logs[0].arn}/*"
      },
      {
        Effect = "Allow"
        Principal = {
          Service = "delivery.logs.amazonaws.com"
        }
        Action   = "s3:PutObject"
        Resource = "${aws_s3_bucket.access_logs[0].arn}/*"
        Condition = {
          StringEquals = {
            "s3:x-amz-acl" = "bucket-owner-full-control"
          }
        }
      },
      {
        Effect = "Allow"
        Principal = {
          Service = "delivery.logs.amazonaws.com"
        }
        Action   = "s3:GetBucketAcl"
        Resource = aws_s3_bucket.access_logs[0].arn
      }
    ]
  })
}

# ELB service account mapping for different regions
locals {
  elb_service_account_mapping = {
    "us-east-1"      = "127311923021"
    "us-east-2"      = "033677994240"
    "us-west-1"      = "027434742980"
    "us-west-2"      = "797873946194"
    "eu-west-1"      = "156460612806"
    "eu-west-2"      = "652711504416"
    "eu-central-1"   = "054676820928"
    "ap-southeast-1" = "114774131450"
    "ap-southeast-2" = "783225319266"
    "ap-northeast-1" = "582318560864"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "access_logs" {
  count  = var.enable_access_logs && var.access_logs_s3_bucket == "" ? 1 : 0
  bucket = aws_s3_bucket.access_logs[0].id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

resource "aws_s3_bucket_lifecycle_configuration" "access_logs" {
  count  = var.enable_access_logs && var.access_logs_s3_bucket == "" ? 1 : 0
  bucket = aws_s3_bucket.access_logs[0].id

  rule {
    id     = "access_logs_lifecycle"
    status = "Enabled"

    expiration {
      days = var.access_logs_retention_days
    }

    noncurrent_version_expiration {
      noncurrent_days = 1
    }

    abort_incomplete_multipart_upload {
      days_after_initiation = 7
    }
  }
}

resource "aws_s3_bucket_public_access_block" "access_logs" {
  count  = var.enable_access_logs && var.access_logs_s3_bucket == "" ? 1 : 0
  bucket = aws_s3_bucket.access_logs[0].id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}