@id("allow-read")
permit (
  principal,
  action == Action::"read",
  resource
);

@id("owner-edit")
permit (
  principal is PhotoApp::User,
  action in [Action::"edit", Action::"share"],
  resource in PhotoApp::Album::"vacation"
)
when { resource.owner == principal };

@id("deny-delete")
@advice("Deletion is disabled")
forbid (principal == PhotoApp::User::"guest", action == Action::"delete", resource)
unless { context.admin };

permit (principal, action == Action::"view", resource is Photo);
