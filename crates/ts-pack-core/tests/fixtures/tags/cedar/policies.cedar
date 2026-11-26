@id("allow-read")
permit (
  principal,
  action == Action::"read",
  resource
);

@id("deny-delete")
forbid (principal, action == Action::"delete", resource);
