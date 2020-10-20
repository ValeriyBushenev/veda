/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.14.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TaskQueryDto : A Task query which defines a group of Tasks.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskQueryDto {
    /// Restrict to tasks that belong to process instances with the given id.
    #[serde(rename = "processInstanceId", skip_serializing_if = "Option::is_none")]
    pub process_instance_id: Option<String>,
    /// Restrict to tasks that belong to process instances with the given ids.
    #[serde(rename = "processInstanceIdIn", skip_serializing_if = "Option::is_none")]
    pub process_instance_id_in: Option<Vec<String>>,
    /// Restrict to tasks that belong to process instances with the given business key.
    #[serde(rename = "processInstanceBusinessKey", skip_serializing_if = "Option::is_none")]
    pub process_instance_business_key: Option<String>,
    /// Restrict to tasks that belong to process instances with the given business key which  is described by an expression. See the  [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions) for more information on available functions.
    #[serde(rename = "processInstanceBusinessKeyExpression", skip_serializing_if = "Option::is_none")]
    pub process_instance_business_key_expression: Option<String>,
    /// Restrict to tasks that belong to process instances with one of the give business keys.  The keys need to be in a comma-separated list.
    #[serde(rename = "processInstanceBusinessKeyIn", skip_serializing_if = "Option::is_none")]
    pub process_instance_business_key_in: Option<Vec<String>>,
    /// Restrict to tasks that have a process instance business key that has the parameter  value as a substring.
    #[serde(rename = "processInstanceBusinessKeyLike", skip_serializing_if = "Option::is_none")]
    pub process_instance_business_key_like: Option<String>,
    /// Restrict to tasks that have a process instance business key that has the parameter  value as a substring and is described by an expression. See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions)  for more information on available functions.
    #[serde(rename = "processInstanceBusinessKeyLikeExpression", skip_serializing_if = "Option::is_none")]
    pub process_instance_business_key_like_expression: Option<String>,
    /// Restrict to tasks that belong to a process definition with the given id.
    #[serde(rename = "processDefinitionId", skip_serializing_if = "Option::is_none")]
    pub process_definition_id: Option<String>,
    /// Restrict to tasks that belong to a process definition with the given key.
    #[serde(rename = "processDefinitionKey", skip_serializing_if = "Option::is_none")]
    pub process_definition_key: Option<String>,
    /// Restrict to tasks that belong to a process definition with one of the given keys. The  keys need to be in a comma-separated list.
    #[serde(rename = "processDefinitionKeyIn", skip_serializing_if = "Option::is_none")]
    pub process_definition_key_in: Option<Vec<String>>,
    /// Restrict to tasks that belong to a process definition with the given name.
    #[serde(rename = "processDefinitionName", skip_serializing_if = "Option::is_none")]
    pub process_definition_name: Option<String>,
    /// Restrict to tasks that have a process definition name that has the parameter value as  a substring.
    #[serde(rename = "processDefinitionNameLike", skip_serializing_if = "Option::is_none")]
    pub process_definition_name_like: Option<String>,
    /// Restrict to tasks that belong to an execution with the given id.
    #[serde(rename = "executionId", skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// Restrict to tasks that belong to case instances with the given id.
    #[serde(rename = "caseInstanceId", skip_serializing_if = "Option::is_none")]
    pub case_instance_id: Option<String>,
    /// Restrict to tasks that belong to case instances with the given business key.
    #[serde(rename = "caseInstanceBusinessKey", skip_serializing_if = "Option::is_none")]
    pub case_instance_business_key: Option<String>,
    /// Restrict to tasks that have a case instance business key that has the parameter value  as a substring.
    #[serde(rename = "caseInstanceBusinessKeyLike", skip_serializing_if = "Option::is_none")]
    pub case_instance_business_key_like: Option<String>,
    /// Restrict to tasks that belong to a case definition with the given id.
    #[serde(rename = "caseDefinitionId", skip_serializing_if = "Option::is_none")]
    pub case_definition_id: Option<String>,
    /// Restrict to tasks that belong to a case definition with the given key.
    #[serde(rename = "caseDefinitionKey", skip_serializing_if = "Option::is_none")]
    pub case_definition_key: Option<String>,
    /// Restrict to tasks that belong to a case definition with the given name.
    #[serde(rename = "caseDefinitionName", skip_serializing_if = "Option::is_none")]
    pub case_definition_name: Option<String>,
    /// Restrict to tasks that have a case definition name that has the parameter value as a  substring.
    #[serde(rename = "caseDefinitionNameLike", skip_serializing_if = "Option::is_none")]
    pub case_definition_name_like: Option<String>,
    /// Restrict to tasks that belong to a case execution with the given id.
    #[serde(rename = "caseExecutionId", skip_serializing_if = "Option::is_none")]
    pub case_execution_id: Option<String>,
    /// Only include tasks which belong to one of the passed and comma-separated activity  instance ids.
    #[serde(rename = "activityInstanceIdIn", skip_serializing_if = "Option::is_none")]
    pub activity_instance_id_in: Option<Vec<String>>,
    /// Only include tasks which belong to one of the passed and comma-separated  tenant ids.
    #[serde(rename = "tenantIdIn", skip_serializing_if = "Option::is_none")]
    pub tenant_id_in: Option<Vec<String>>,
    /// Only include tasks which belong to no tenant. Value may only be `true`,  as `false` is the default behavior.
    #[serde(rename = "withoutTenantId", skip_serializing_if = "Option::is_none")]
    pub without_tenant_id: Option<bool>,
    /// Restrict to tasks that the given user is assigned to.
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    /// Restrict to tasks that the user described by the given expression is assigned to. See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions)  for more information on available functions.
    #[serde(rename = "assigneeExpression", skip_serializing_if = "Option::is_none")]
    pub assignee_expression: Option<String>,
    /// Restrict to tasks that have an assignee that has the parameter  value as a substring.
    #[serde(rename = "assigneeLike", skip_serializing_if = "Option::is_none")]
    pub assignee_like: Option<String>,
    /// Restrict to tasks that have an assignee that has the parameter value described by the  given expression as a substring. See the  [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions)  for more information on available functions.
    #[serde(rename = "assigneeLikeExpression", skip_serializing_if = "Option::is_none")]
    pub assignee_like_expression: Option<String>,
    /// Only include tasks which are assigned to one of the passed and comma-separated user ids.
    #[serde(rename = "assigneeIn", skip_serializing_if = "Option::is_none")]
    pub assignee_in: Option<Vec<String>>,
    /// Restrict to tasks that the given user owns.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Restrict to tasks that the user described by the given expression owns. See the  [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions)  for more information on available functions.
    #[serde(rename = "ownerExpression", skip_serializing_if = "Option::is_none")]
    pub owner_expression: Option<String>,
    /// Only include tasks that are offered to the given group.
    #[serde(rename = "candidateGroup", skip_serializing_if = "Option::is_none")]
    pub candidate_group: Option<String>,
    /// Only include tasks that are offered to the group described by the given expression.  See the  [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions)  for more information on available functions.
    #[serde(rename = "candidateGroupExpression", skip_serializing_if = "Option::is_none")]
    pub candidate_group_expression: Option<String>,
    /// Only include tasks that are offered to the given user or to one of his groups.
    #[serde(rename = "candidateUser", skip_serializing_if = "Option::is_none")]
    pub candidate_user: Option<String>,
    /// Only include tasks that are offered to the user described by the given expression.  See the  [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions)  for more information on available functions.
    #[serde(rename = "candidateUserExpression", skip_serializing_if = "Option::is_none")]
    pub candidate_user_expression: Option<String>,
    /// Also include tasks that are assigned to users in candidate queries. Default is to only  include tasks that are not assigned to any user if you query by candidate user or group(s).
    #[serde(rename = "includeAssignedTasks", skip_serializing_if = "Option::is_none")]
    pub include_assigned_tasks: Option<bool>,
    /// Only include tasks that the given user is involved in. A user is involved in a task if  an identity link exists between task and user (e.g., the user is the assignee).
    #[serde(rename = "involvedUser", skip_serializing_if = "Option::is_none")]
    pub involved_user: Option<String>,
    /// Only include tasks that the user described by the given expression is involved in. A user is involved in a task if an identity link exists between task and user (e.g., the user is the assignee). See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions) for more information on available functions.
    #[serde(rename = "involvedUserExpression", skip_serializing_if = "Option::is_none")]
    pub involved_user_expression: Option<String>,
    /// If set to `true`, restricts the query to all tasks that are assigned.
    #[serde(rename = "assigned", skip_serializing_if = "Option::is_none")]
    pub assigned: Option<bool>,
    /// If set to `true`, restricts the query to all tasks that are unassigned.
    #[serde(rename = "unassigned", skip_serializing_if = "Option::is_none")]
    pub unassigned: Option<bool>,
    /// Restrict to tasks that have the given key.
    #[serde(rename = "taskDefinitionKey", skip_serializing_if = "Option::is_none")]
    pub task_definition_key: Option<String>,
    /// Restrict to tasks that have one of the given keys. The keys need to be in a comma-separated list.
    #[serde(rename = "taskDefinitionKeyIn", skip_serializing_if = "Option::is_none")]
    pub task_definition_key_in: Option<Vec<String>>,
    /// Restrict to tasks that have a key that has the parameter value as a substring.
    #[serde(rename = "taskDefinitionKeyLike", skip_serializing_if = "Option::is_none")]
    pub task_definition_key_like: Option<String>,
    /// Restrict to tasks that have the given name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Restrict to tasks that do not have the given name.
    #[serde(rename = "nameNotEqual", skip_serializing_if = "Option::is_none")]
    pub name_not_equal: Option<String>,
    /// Restrict to tasks that have a name with the given parameter value as substring.
    #[serde(rename = "nameLike", skip_serializing_if = "Option::is_none")]
    pub name_like: Option<String>,
    /// Restrict to tasks that do not have a name with the given parameter value as substring.
    #[serde(rename = "nameNotLike", skip_serializing_if = "Option::is_none")]
    pub name_not_like: Option<String>,
    /// Restrict to tasks that have the given description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Restrict to tasks that have a description that has the parameter value as a substring.
    #[serde(rename = "descriptionLike", skip_serializing_if = "Option::is_none")]
    pub description_like: Option<String>,
    /// Restrict to tasks that have the given priority.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Restrict to tasks that have a lower or equal priority.
    #[serde(rename = "maxPriority", skip_serializing_if = "Option::is_none")]
    pub max_priority: Option<i32>,
    /// Restrict to tasks that have a higher or equal priority.
    #[serde(rename = "minPriority", skip_serializing_if = "Option::is_none")]
    pub min_priority: Option<i32>,
    /// Restrict to tasks that are due on the given date. By [default](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/), the date must have the format `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, e.g., `2013-01-23T14:42:45.546+0200`.
    #[serde(rename = "dueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    /// Restrict to tasks that are due on the date described by the given expression. See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions) for more information on available functions. The expression must evaluate to a `java.util.Date` or `org.joda.time.DateTime` object.
    #[serde(rename = "dueDateExpression", skip_serializing_if = "Option::is_none")]
    pub due_date_expression: Option<String>,
    /// Restrict to tasks that are due after the given date. By [default](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/), the date must have the format `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, e.g., `2013-01-23T14:42:45.435+0200`.
    #[serde(rename = "dueAfter", skip_serializing_if = "Option::is_none")]
    pub due_after: Option<String>,
    /// Restrict to tasks that are due after the date described by the given expression. See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions) for more information on available functions. The expression must evaluate to a `java.util.Date` or `org.joda.time.DateTime` object.
    #[serde(rename = "dueAfterExpression", skip_serializing_if = "Option::is_none")]
    pub due_after_expression: Option<String>,
    /// Restrict to tasks that are due before the given date. By [default](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/), the date must have the format `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, e.g., `2013-01-23T14:42:45.243+0200`.
    #[serde(rename = "dueBefore", skip_serializing_if = "Option::is_none")]
    pub due_before: Option<String>,
    /// Restrict to tasks that are due before the date described by the given expression. See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions) for more information on available functions. The expression must evaluate to a `java.util.Date` or `org.joda.time.DateTime` object.
    #[serde(rename = "dueBeforeExpression", skip_serializing_if = "Option::is_none")]
    pub due_before_expression: Option<String>,
    /// Restrict to tasks that have a followUp date on the given date. By [default](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/), the date must have the format `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, e.g., `2013-01-23T14:42:45.342+0200`.
    #[serde(rename = "followUpDate", skip_serializing_if = "Option::is_none")]
    pub follow_up_date: Option<String>,
    /// Restrict to tasks that have a followUp date on the date described by the given expression. See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions) for more information on available functions. The expression must evaluate to a `java.util.Date` or `org.joda.time.DateTime` object.
    #[serde(rename = "followUpDateExpression", skip_serializing_if = "Option::is_none")]
    pub follow_up_date_expression: Option<String>,
    /// Restrict to tasks that have a followUp date after the given date. By [default](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/), the date must have the format `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, e.g., `2013-01-23T14:42:45.542+0200`.
    #[serde(rename = "followUpAfter", skip_serializing_if = "Option::is_none")]
    pub follow_up_after: Option<String>,
    /// Restrict to tasks that have a followUp date after the date described by the given expression. See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions) for more information on available functions. The expression must evaluate to a `java.util.Date` or `org.joda.time.DateTime` object.
    #[serde(rename = "followUpAfterExpression", skip_serializing_if = "Option::is_none")]
    pub follow_up_after_expression: Option<String>,
    /// Restrict to tasks that have a followUp date before the given date. By [default](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/), the date must have the format `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, e.g., `2013-01-23T14:42:45.234+0200`.
    #[serde(rename = "followUpBefore", skip_serializing_if = "Option::is_none")]
    pub follow_up_before: Option<String>,
    /// Restrict to tasks that have a followUp date before the date described by the given expression. See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions) for more information on available functions. The expression must evaluate to a `java.util.Date` or `org.joda.time.DateTime` object.
    #[serde(rename = "followUpBeforeExpression", skip_serializing_if = "Option::is_none")]
    pub follow_up_before_expression: Option<String>,
    /// Restrict to tasks that have no followUp date or a followUp date before the given date. By [default](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/), the date must have the format `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, e.g., `2013-01-23T14:42:45.432+0200`. The typical use case is to query all `active` tasks for a user for a given date.
    #[serde(rename = "followUpBeforeOrNotExistent", skip_serializing_if = "Option::is_none")]
    pub follow_up_before_or_not_existent: Option<String>,
    /// Restrict to tasks that have no followUp date or a followUp date before the date described by the given expression. See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions) for more information on available functions. The expression must evaluate to a `java.util.Date` or `org.joda.time.DateTime` object.
    #[serde(rename = "followUpBeforeOrNotExistentExpression", skip_serializing_if = "Option::is_none")]
    pub follow_up_before_or_not_existent_expression: Option<String>,
    /// Restrict to tasks that were created on the given date. By [default](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/), the date must have the format `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, e.g., `2013-01-23T14:42:45.324+0200`.
    #[serde(rename = "createdOn", skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    /// Restrict to tasks that were created on the date described by the given expression. See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions) for more information on available functions. The expression must evaluate to a `java.util.Date` or `org.joda.time.DateTime` object.
    #[serde(rename = "createdOnExpression", skip_serializing_if = "Option::is_none")]
    pub created_on_expression: Option<String>,
    /// Restrict to tasks that were created after the given date. By [default](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/), the date must have the format `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, e.g., `2013-01-23T14:42:45.342+0200`.
    #[serde(rename = "createdAfter", skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Restrict to tasks that were created after the date described by the given expression. See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions) for more information on available functions. The expression must evaluate to a `java.util.Date` or `org.joda.time.DateTime` object.
    #[serde(rename = "createdAfterExpression", skip_serializing_if = "Option::is_none")]
    pub created_after_expression: Option<String>,
    /// Restrict to tasks that were created before the given date. By [default](https://docs.camunda.org/manual/7.14/reference/rest/overview/date-format/), the date must have the format `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, e.g., `2013-01-23T14:42:45.332+0200`.
    #[serde(rename = "createdBefore", skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Restrict to tasks that were created before the date described by the given expression. See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions) for more information on available functions. The expression must evaluate to a `java.util.Date` or `org.joda.time.DateTime` object.
    #[serde(rename = "createdBeforeExpression", skip_serializing_if = "Option::is_none")]
    pub created_before_expression: Option<String>,
    /// Restrict to tasks that are in the given delegation state. Valid values are `PENDING` and `RESOLVED`.
    #[serde(rename = "delegationState", skip_serializing_if = "Option::is_none")]
    pub delegation_state: Option<DelegationState>,
    /// Restrict to tasks that are offered to any of the given candidate groups. Takes a comma-separated list of group names, so for example `developers,support,sales`.
    #[serde(rename = "candidateGroups", skip_serializing_if = "Option::is_none")]
    pub candidate_groups: Option<Vec<String>>,
    /// Restrict to tasks that are offered to any of the candidate groups described by the given expression. See the [user guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/expression-language/#internal-context-functions) for more information on available functions. The expression must evaluate to `java.util.List` of Strings.
    #[serde(rename = "candidateGroupsExpression", skip_serializing_if = "Option::is_none")]
    pub candidate_groups_expression: Option<String>,
    /// Only include tasks which have a candidate group. Value may only be `true`, as `false` is the default behavior.
    #[serde(rename = "withCandidateGroups", skip_serializing_if = "Option::is_none")]
    pub with_candidate_groups: Option<bool>,
    /// Only include tasks which have no candidate group. Value may only be `true`, as `false` is the default behavior.
    #[serde(rename = "withoutCandidateGroups", skip_serializing_if = "Option::is_none")]
    pub without_candidate_groups: Option<bool>,
    /// Only include tasks which have a candidate user. Value may only be `true`, as `false` is the default behavior.
    #[serde(rename = "withCandidateUsers", skip_serializing_if = "Option::is_none")]
    pub with_candidate_users: Option<bool>,
    /// Only include tasks which have no candidate users. Value may only be `true`, as `false` is the default behavior.
    #[serde(rename = "withoutCandidateUsers", skip_serializing_if = "Option::is_none")]
    pub without_candidate_users: Option<bool>,
    /// Only include active tasks. Value may only be `true`, as `false` is the default behavior.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Only include suspended tasks. Value may only be `true`, as `false` is the default behavior.
    #[serde(rename = "suspended", skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    /// A JSON array to only include tasks that have variables with certain values. The array consists of JSON objects with three properties `name`, `operator` and `value`. `name` is the variable name, `operator` is the comparison operator to be used and `value` the variable value. `value` may be of type `String`, `Number` or `Boolean`.  Valid `operator` values are: `eq` - equal to; `neq` - not equal to; `gt` - greater than; `gteq` - greater than or equal to; `lt` - lower than; `lteq` - lower than or equal to; `like`. `key` and `value` may not contain underscore or comma characters.
    #[serde(rename = "taskVariables", skip_serializing_if = "Option::is_none")]
    pub task_variables: Option<Vec<crate::models::VariableQueryParameterDto>>,
    /// A JSON array to only include tasks that belong to a process instance with variables with certain values. The array consists of JSON objects with three properties `name`, `operator` and `value`. `name` is the variable name, `operator` is the comparison operator to be used and `value` the variable value. `value` may be of type `String`, `Number` or `Boolean`.  Valid `operator` values are: `eq` - equal to; `neq` - not equal to; `gt` - greater than; `gteq` - greater than or equal to; `lt` - lower than; `lteq` - lower than or equal to; `like`. `key` and `value` may not contain underscore or comma characters.
    #[serde(rename = "processVariables", skip_serializing_if = "Option::is_none")]
    pub process_variables: Option<Vec<crate::models::VariableQueryParameterDto>>,
    /// A JSON array to only include tasks that belong to a case instance with variables with certain values. The array consists of JSON objects with three properties `name`, `operator` and `value`. `name` is the variable name, `operator` is the comparison operator to be used and `value` the variable value. `value` may be of type `String`, `Number` or `Boolean`.  Valid `operator` values are: `eq` - equal to; `neq` - not equal to; `gt` - greater than; `gteq` - greater than or equal to; `lt` - lower than; `lteq` - lower than or equal to; `like`. `key` and `value` may not contain underscore or comma characters.
    #[serde(rename = "caseInstanceVariables", skip_serializing_if = "Option::is_none")]
    pub case_instance_variables: Option<Vec<crate::models::VariableQueryParameterDto>>,
    /// Match all variable names in this query case-insensitively. If set `variableName` and `variablename` are treated as equal.
    #[serde(rename = "variableNamesIgnoreCase", skip_serializing_if = "Option::is_none")]
    pub variable_names_ignore_case: Option<bool>,
    /// Match all variable values in this query case-insensitively. If set `variableValue` and `variablevalue` are treated as equal.
    #[serde(rename = "variableValuesIgnoreCase", skip_serializing_if = "Option::is_none")]
    pub variable_values_ignore_case: Option<bool>,
    /// Restrict query to all tasks that are sub tasks of the given task. Takes a task id.
    #[serde(rename = "parentTaskId", skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<String>,
    /// A JSON array of nested task queries with OR semantics. A task matches a nested query if it fulfills *at least one* of the query's predicates. With multiple nested queries, a task must fulfill at least one predicate of *each* query ([Conjunctive Normal Form](https://en.wikipedia.org/wiki/Conjunctive_normal_form)).  All task query properties can be used except for: `sorting`, `withCandidateGroups`, `withoutCandidateGroups`, `withCandidateUsers`, `withoutCandidateUsers`  See the [User guide](https://docs.camunda.org/manual/7.14/user-guide/process-engine/process-engine-api/#or-queries) for more information about OR queries.
    #[serde(rename = "orQueries", skip_serializing_if = "Option::is_none")]
    pub or_queries: Option<Vec<crate::models::TaskQueryDto>>,
    /// Apply sorting of the result
    #[serde(rename = "sorting", skip_serializing_if = "Option::is_none")]
    pub sorting: Option<Vec<crate::models::TaskQueryDtoSorting>>,
}

impl TaskQueryDto {
    /// A Task query which defines a group of Tasks.
    pub fn new() -> TaskQueryDto {
        TaskQueryDto {
            process_instance_id: None,
            process_instance_id_in: None,
            process_instance_business_key: None,
            process_instance_business_key_expression: None,
            process_instance_business_key_in: None,
            process_instance_business_key_like: None,
            process_instance_business_key_like_expression: None,
            process_definition_id: None,
            process_definition_key: None,
            process_definition_key_in: None,
            process_definition_name: None,
            process_definition_name_like: None,
            execution_id: None,
            case_instance_id: None,
            case_instance_business_key: None,
            case_instance_business_key_like: None,
            case_definition_id: None,
            case_definition_key: None,
            case_definition_name: None,
            case_definition_name_like: None,
            case_execution_id: None,
            activity_instance_id_in: None,
            tenant_id_in: None,
            without_tenant_id: None,
            assignee: None,
            assignee_expression: None,
            assignee_like: None,
            assignee_like_expression: None,
            assignee_in: None,
            owner: None,
            owner_expression: None,
            candidate_group: None,
            candidate_group_expression: None,
            candidate_user: None,
            candidate_user_expression: None,
            include_assigned_tasks: None,
            involved_user: None,
            involved_user_expression: None,
            assigned: None,
            unassigned: None,
            task_definition_key: None,
            task_definition_key_in: None,
            task_definition_key_like: None,
            name: None,
            name_not_equal: None,
            name_like: None,
            name_not_like: None,
            description: None,
            description_like: None,
            priority: None,
            max_priority: None,
            min_priority: None,
            due_date: None,
            due_date_expression: None,
            due_after: None,
            due_after_expression: None,
            due_before: None,
            due_before_expression: None,
            follow_up_date: None,
            follow_up_date_expression: None,
            follow_up_after: None,
            follow_up_after_expression: None,
            follow_up_before: None,
            follow_up_before_expression: None,
            follow_up_before_or_not_existent: None,
            follow_up_before_or_not_existent_expression: None,
            created_on: None,
            created_on_expression: None,
            created_after: None,
            created_after_expression: None,
            created_before: None,
            created_before_expression: None,
            delegation_state: None,
            candidate_groups: None,
            candidate_groups_expression: None,
            with_candidate_groups: None,
            without_candidate_groups: None,
            with_candidate_users: None,
            without_candidate_users: None,
            active: None,
            suspended: None,
            task_variables: None,
            process_variables: None,
            case_instance_variables: None,
            variable_names_ignore_case: None,
            variable_values_ignore_case: None,
            parent_task_id: None,
            or_queries: None,
            sorting: None,
        }
    }
}

/// Restrict to tasks that are in the given delegation state. Valid values are `PENDING` and `RESOLVED`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DelegationState {
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "RESOLVED")]
    RESOLVED,
}

