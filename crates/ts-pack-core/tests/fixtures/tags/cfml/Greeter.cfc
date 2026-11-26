<cfcomponent name="Greeter">
  <cffunction name="greet" returntype="string">
    <cfargument name="who" type="string">
    <cfreturn "Hello " & who>
  </cffunction>
</cfcomponent>
