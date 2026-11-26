terraform {
  required_version = ">= 1.5"
}

provider "aws" {
  region = "eu-west-1"
}

resource "aws_s3_bucket" "logs" {
  bucket = "logs"
}

data "aws_ami" "ubuntu" {
  most_recent = true
}

variable "env" {
  type = string
}

output "bucket_arn" {
  value = aws_s3_bucket.logs.arn
}

module "vpc" {
  source = "./vpc"
}

locals {
  name_prefix = "app"
  tags        = {}
}

include {
  path = find_in_parent_folders()
}

include "root" {
  path = "root.hcl"
}

dependency "network" {
  config_path = "../network"
}

locals {
  region = "eu-west-1"
}

inputs = {
  vpc_id = dependency.network.outputs.vpc_id
}
