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
