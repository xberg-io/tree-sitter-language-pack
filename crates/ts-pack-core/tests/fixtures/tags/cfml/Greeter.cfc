<cfcomponent name="Greeter" extends="Base" output="false">
  <cfset variables.count = 0>

  <cffunction name="init" returntype="Greeter" access="public">
    <cfset variables.count = 0>
    <cfreturn this>
  </cffunction>

  <cffunction name="greet" returntype="string" access="public">
    <cfargument name="who" type="string" required="true">
    <cfset var msg = "Hello " & arguments.who>
    <cfset variables.count = increment(variables.count)>
    <cfreturn ucase(msg)>
  </cffunction>

  <cffunction name="greetAll" returntype="array" access="public">
    <cfargument name="names" type="array" required="true">
    <cfset var result = arrayMap(arguments.names, function(n) { return trim(n); })>
    <cfreturn this.greet(arrayToList(result))>
  </cffunction>

  <cffunction name="increment" returntype="numeric" access="private">
    <cfargument name="value" type="numeric" required="true">
    <cfreturn arguments.value + 1>
  </cffunction>
</cfcomponent>
