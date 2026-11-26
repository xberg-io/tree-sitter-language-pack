local defaults = import 'defaults.libsonnet';

local port = 8080;

local container(name, image) = {
  name: name,
  image: image,
  ports: [{ containerPort: port }],
};

{
  replicas:: 3,
  podTemplate(name):: { metadata: { name: name } },
  deployment(name):: {
    apiVersion: 'apps/v1',
    kind: 'Deployment',
    spec: {
      replicas: $.replicas,
      template: $.podTemplate(name),
      containers: [container(name, defaults.image)],
    },
  },
}
