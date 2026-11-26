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

dependency "vpc" {
  config_path = "../vpc"
}

locals {
  region = "eu-west-1"
}

inputs = {
  vpc_id = dependency.vpc.outputs.vpc_id
}
