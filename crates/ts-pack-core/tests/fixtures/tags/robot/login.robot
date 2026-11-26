*** Settings ***
Library    SeleniumLibrary

*** Variables ***
${URL}    https://example.com
@{USERS}    alice    bob
&{OPTIONS}    browser=firefox    headless=true

*** Test Cases ***
Valid Login
    Open Login Page
    Submit Credentials    alice    secret

*** Keywords ***
Open Login Page
    Open Browser    ${URL}

Submit Credentials
    [Arguments]    ${user}    ${password}
    Input Text    username    ${user}
