# Ingress Module Outputs

# Certificate ARN for SSL termination
output "certificate_arn" {
  description = "ARN of the ACM certificate for SSL termination"
  value       = aws_acm_certificate.main.arn
}

output "certificate_domain" {
  description = "Domain name of the SSL certificate"
  value       = aws_acm_certificate.main.domain_name
}

# Load Balancer Security Group
output "alb_security_group_id" {
  description = "ID of the ALB security group"
  value       = aws_security_group.alb.id
}

output "alb_security_group_arn" {
  description = "ARN of the ALB security group"
  value       = aws_security_group.alb.arn
}

# AWS Load Balancer Controller IAM Role
output "load_balancer_controller_role_arn" {
  description = "ARN of the AWS Load Balancer Controller IAM role"
  value       = aws_iam_role.aws_load_balancer_controller.arn
}

output "load_balancer_controller_role_name" {
  description = "Name of the AWS Load Balancer Controller IAM role"
  value       = aws_iam_role.aws_load_balancer_controller.name
}

# Kubernetes Resources
output "ingress_namespace" {
  description = "Name of the ingress namespace"
  value       = kubernetes_namespace.ingress.metadata[0].name
}

output "load_balancer_controller_service_account" {
  description = "Name of the AWS Load Balancer Controller service account"
  value       = kubernetes_service_account.aws_load_balancer_controller.metadata[0].name
}

output "ingress_class_name" {
  description = "Name of the Kubernetes IngressClass"
  value       = kubernetes_ingress_class_v1.alb.metadata[0].name
}

# Default Backend
output "default_backend_service_name" {
  description = "Name of the default backend service"
  value       = kubernetes_service.default_backend.metadata[0].name
}

output "default_backend_service_namespace" {
  description = "Namespace of the default backend service"
  value       = kubernetes_service.default_backend.metadata[0].namespace
}

# Access Logs S3 Bucket
output "access_logs_bucket_name" {
  description = "Name of the S3 bucket for ALB access logs"
  value       = aws_s3_bucket.access_logs.bucket
}

output "access_logs_bucket_arn" {
  description = "ARN of the S3 bucket for ALB access logs"
  value       = aws_s3_bucket.access_logs.arn
}

output "access_logs_bucket_domain_name" {
  description = "Bucket domain name for access logs"
  value       = aws_s3_bucket.access_logs.bucket_domain_name
}

# Helm Release
output "aws_load_balancer_controller_chart_version" {
  description = "Version of the AWS Load Balancer Controller Helm chart"
  value       = helm_release.aws_load_balancer_controller.version
}

output "aws_load_balancer_controller_status" {
  description = "Status of the AWS Load Balancer Controller Helm release"
  value       = helm_release.aws_load_balancer_controller.status
}

# Route 53
output "route53_zone_id" {
  description = "ID of the Route53 hosted zone"
  value       = data.aws_route53_zone.main.zone_id
  sensitive   = false
}

output "route53_zone_name" {
  description = "Name of the Route53 hosted zone"
  value       = data.aws_route53_zone.main.name
}

# Certificate Validation
output "certificate_validation_records" {
  description = "Certificate validation DNS records"
  value       = aws_route53_record.certificate_validation[*].fqdn
}

output "certificate_status" {
  description = "Status of the ACM certificate"
  value       = aws_acm_certificate.main.status
}

# Security Group Rules
output "alb_security_group_rules" {
  description = "List of security group rule IDs for the ALB"
  value       = aws_security_group_rule.alb_additional[*].id
}