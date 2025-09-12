#!/bin/bash
# User data script for EKS worker nodes
# This script bootstraps the node to join the EKS cluster

set -o xtrace
/etc/eks/bootstrap.sh ${cluster_name} ${bootstrap_arguments}
/opt/aws/bin/cfn-signal --exit-code $? --stack ${AWS::StackName} --resource NodeGroup --region ${AWS::Region}