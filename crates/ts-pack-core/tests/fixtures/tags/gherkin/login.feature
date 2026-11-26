Feature: Login

  Background:
    Given a registered user

  Scenario: Successful login
    When the user logs in
    Then the dashboard is shown

  Scenario: Failed login
    When the user enters a wrong password
    Then an error is shown
