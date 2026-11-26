dbo.RefreshStats 'Users';
GO

EXEC dbo.GetUser @Id = 1;
GO

EXECUTE sp_rename 'dbo.Users', 'Members';
GO

SELECT Name, LEN(Name) AS NameLength, @@VERSION AS ServerVersion
FROM dbo.Users;
GO
