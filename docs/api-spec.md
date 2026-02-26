# Singularity API v2 — Full Specification

Base URL: `https://api.singularity-app.com`

## Authentication

All endpoints require a bearer token in the `Authorization` header.

```
Authorization: Bearer <token>
```

---

## Endpoints

### Projects `/v2/project`

#### GET `/v2/project` — List projects

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| maxCount | number | no | Maximum results (max 1000) |
| offset | number | no | Pagination offset (min 0) |
| includeRemoved | boolean | no | Include soft-deleted (default false) |
| includeArchived | boolean | no | Include archived (default false) |

**Response:** `ProjectListResponseDto`

#### POST `/v2/project` — Create project

**Body:** `ProjectCreateDto`
**Response:** `ProjectCreateResponseDto` (contains `project` + auto-created `taskGroup`)

#### GET `/v2/project/{id}` — Get project by ID

**Response:** `ProjectResponseDto`

#### PATCH `/v2/project/{id}` — Update project

**Body:** `ProjectUpdateDto`
**Response:** `ProjectResponseDto`

#### DELETE `/v2/project/{id}` — Delete project

**Response:** 204 No Content

---

### Tasks `/v2/task`

#### GET `/v2/task` — List tasks

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| maxCount | number | no | Maximum results (max 1000) |
| offset | number | no | Pagination offset (min 0) |
| includeRemoved | boolean | no | Include soft-deleted (default false) |
| includeArchived | boolean | no | Include archived (default false) |
| includeAllRecurrenceInstances | boolean | no | Include all recurrence instances (default false) |
| projectId | string | no | Filter by project ID (P-uuid) |
| parent | string | no | Filter by parent task (T-uuid) |
| startDateFrom | string | no | Start date range start, inclusive (ISO 8601) |
| startDateTo | string | no | Start date range end, inclusive (ISO 8601) |

**Response:** `TaskListResponseDto`

#### POST `/v2/task` — Create task

**Body:** `TaskCreateDto`
**Response:** `TaskResponseDto`

#### GET `/v2/task/{id}` — Get task by ID

**Response:** `TaskResponseDto`

#### PATCH `/v2/task/{id}` — Update task

**Body:** `TaskUpdateDto`
**Response:** `TaskResponseDto`

#### DELETE `/v2/task/{id}` — Delete task

**Response:** 204 No Content

---

### Task Groups `/v2/task-group`

#### GET `/v2/task-group` — List task groups

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| maxCount | number | no | Maximum results (max 1000) |
| offset | number | no | Pagination offset (min 0) |
| includeRemoved | boolean | no | Include soft-deleted (default false) |
| parent | string | no | Filter by parent project (P-uuid) |

**Response:** `TaskGroupListResponseDto`

#### POST `/v2/task-group` — Create task group

**Body:** `TaskGroupCreateDto`
**Response:** `TaskGroupResponseDto`

#### GET `/v2/task-group/{id}` — Get task group by ID

**Response:** `TaskGroupResponseDto`

#### PATCH `/v2/task-group/{id}` — Update task group

**Body:** `TaskGroupUpdateDto`
**Response:** `TaskGroupResponseDto`

#### DELETE `/v2/task-group/{id}` — Delete task group

**Response:** 204 No Content

---

### Kanban Statuses `/v2/kanban-status`

#### GET `/v2/kanban-status` — List kanban statuses

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| maxCount | number | no | Maximum results (max 1000) |
| offset | number | no | Pagination offset (min 0) |
| includeRemoved | boolean | no | Include soft-deleted (default false) |
| projectId | string | no | Filter by project (P-uuid) |

**Response:** `KanbanStatusListResponseDto`

#### POST `/v2/kanban-status` — Create kanban status

**Body:** `KanbanStatusCreateDto`
**Response:** `KanbanStatusResponseDto`

#### GET `/v2/kanban-status/{id}` — Get by ID

**Response:** `KanbanStatusResponseDto`

#### PATCH `/v2/kanban-status/{id}` — Update

**Body:** `KanbanStatusUpdateDto`
**Response:** `KanbanStatusResponseDto`

#### DELETE `/v2/kanban-status/{id}` — Delete

**Response:** 204 No Content

---

### Kanban Task Status `/v2/kanban-task-status`

#### GET `/v2/kanban-task-status` — List task-status links

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| maxCount | number | no | Maximum results (max 1000) |
| offset | number | no | Pagination offset (min 0) |
| includeRemoved | boolean | no | Include soft-deleted (default false) |
| taskId | string | no | Filter by task (T-uuid) |
| statusId | string | no | Filter by kanban status (KS-uuid) |

**Response:** `KanbanTaskStatusListResponseDto`

#### POST `/v2/kanban-task-status` — Create link

**Body:** `KanbanTaskStatusCreateDto`
**Response:** `KanbanTaskStatusResponseDto`

#### GET `/v2/kanban-task-status/{id}` — Get by ID

**Response:** `KanbanTaskStatusResponseDto`

#### PATCH `/v2/kanban-task-status/{id}` — Update

**Body:** `KanbanTaskStatusUpdateDto`
**Response:** `KanbanTaskStatusResponseDto`

#### DELETE `/v2/kanban-task-status/{id}` — Delete

**Response:** 204 No Content

---

### Habits `/v2/habit`

#### GET `/v2/habit` — List habits

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| maxCount | number | no | Maximum results (max 1000) |
| offset | number | no | Pagination offset (min 0) |

**Response:** `HabitListResponseDto`

#### POST `/v2/habit` — Create habit

**Body:** `HabitCreateDto`
**Response:** `HabitResponseDto`

#### GET `/v2/habit/{id}` — Get by ID

**Response:** `HabitResponseDto`

#### PATCH `/v2/habit/{id}` — Update

**Body:** `HabitUpdateDto`
**Response:** `HabitResponseDto`

#### DELETE `/v2/habit/{id}` — Delete

**Response:** 204 No Content

---

### Habit Progress `/v2/habit-progress`

#### GET `/v2/habit-progress` — List progress records

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| maxCount | number | no | Maximum results (max 1000) |
| offset | number | no | Pagination offset (min 0) |
| habit | string | no | Filter by habit ID |
| startDate | string | no | Period start date (ISO 8601) |
| endDate | string | no | Period end date (ISO 8601) |

**Response:** `HabitDailyProgressListResponseDto`

#### POST `/v2/habit-progress` — Create progress record

**Body:** `HabitDailyProgressCreateDto`
**Response:** `HabitDailyProgressResponseDto`

#### GET `/v2/habit-progress/{id}` — Get by ID

**Response:** `HabitDailyProgressResponseDto`

#### PATCH `/v2/habit-progress/{id}` — Update

**Body:** `HabitDailyProgressUpdateDto`
**Response:** `HabitDailyProgressResponseDto`

#### DELETE `/v2/habit-progress/{id}` — Delete

**Response:** 204 No Content

---

### Checklist Items `/v2/checklist-item`

#### GET `/v2/checklist-item` — List checklist items

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| maxCount | number | no | Maximum results (max 1000) |
| offset | number | no | Pagination offset (min 0) |
| includeRemoved | boolean | no | Include soft-deleted (default false) |
| parent | string | no | Filter by parent entity ID |

**Response:** `ChecklistItemListResponseDto`

#### POST `/v2/checklist-item` — Create

**Body:** `ChecklistItemCreateDto`
**Response:** `ChecklistItemResponseDto`

#### GET `/v2/checklist-item/{id}` — Get by ID

**Response:** `ChecklistItemResponseDto`

#### PATCH `/v2/checklist-item/{id}` — Update

**Body:** `ChecklistItemUpdateDto`
**Response:** `ChecklistItemResponseDto`

#### DELETE `/v2/checklist-item/{id}` — Delete

**Response:** 204 No Content

---

### Tags `/v2/tag`

#### GET `/v2/tag` — List tags

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| maxCount | number | no | Maximum results (max 1000) |
| offset | number | no | Pagination offset (min 0) |
| includeRemoved | boolean | no | Include soft-deleted (default false) |
| parent | string | no | Filter by parent tag ID |

**Response:** `TagListResponseDto`

#### POST `/v2/tag` — Create tag

**Body:** `TagCreateDto`
**Response:** `TagResponseDto`

#### GET `/v2/tag/{id}` — Get by ID

**Response:** `TagResponseDto`

#### PATCH `/v2/tag/{id}` — Update

**Body:** `TagUpdateDto`
**Response:** `TagResponseDto`

#### DELETE `/v2/tag/{id}` — Delete

**Response:** 204 No Content

---

### Time Stats `/v2/time-stat`

#### GET `/v2/time-stat` — List time stats

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| maxCount | number | no | Maximum results (max 1000) |
| offset | number | no | Pagination offset (min 0) |
| dateFrom | string | no | Filter by start date (from) |
| dateTo | string | no | Filter by start date (to) |
| relatedTaskId | string | no | Filter by related task ID |

**Response:** `TimeStatListResponseDto`

#### POST `/v2/time-stat` — Create time stat

**Body:** `TimeStatCreateDto`
**Response:** `TimeStatResponseDto`

#### DELETE `/v2/time-stat` — Bulk delete

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| dateFrom | string | no | Filter by date (from) |
| dateTo | string | no | Filter by date (to) |
| relatedTaskId | string | no | Filter by related task ID |

**Response:** `TimeStatDeleteBulkResponseDto` (`count`: number)

#### GET `/v2/time-stat/{id}` — Get by ID

**Response:** `TimeStatResponseDto`

#### PATCH `/v2/time-stat/{id}` — Update

**Body:** `TimeStatUpdateDto`
**Response:** `TimeStatResponseDto`

#### DELETE `/v2/time-stat/{id}` — Delete

**Response:** 204 No Content

---

## Schemas

### Enums

#### TaskPriority
| Value | Meaning |
|-------|---------|
| 0 | HIGH |
| 1 | NORMAL |
| 2 | LOW |

#### TaskState
| Value | Meaning |
|-------|---------|
| 0 | PINNED |
| 1 | UNPINNED |

#### TaskCheck
| Value | Meaning |
|-------|---------|
| 0 | EMPTY |
| 1 | CHECKED |
| 2 | CANCELLED |

#### ReviewValidationInterval
Values: `0`, `1`, `2`, `3`, `4`, `-1`

#### HabitStatus
Values: `0`, `1`, `2`, `3`

#### HabitColor
`red`, `pink`, `purple`, `deepPurple`, `indigo`, `lightBlue`, `cyan`, `teal`, `green`, `lightGreen`, `lime`, `yellow`, `amber`, `orange`, `deepOrange`, `brown`, `grey`, `blueGrey`

### ID Formats

| Entity | Prefix | Example |
|--------|--------|---------|
| Project | P- | `P-a1b2c3d4-e5f6-...` |
| Task | T- | `T-f7e8d9c0-b1a2-...` |
| Task Group | Q- | `Q-1a2b3c4d-5e6f-...` |
| Kanban Status | KS- | `KS-abcd1234-...` |
| Kanban Task Status | KTS- | `KTS-abcd1234-...` |
| Checklist Item | CH- | `CH-033c36d1-bab5-...` |

---

### ProjectResponseDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | yes | Project identifier (P-uuid) |
| title | string | yes | Project title |
| note | string | no | Project note |
| start | string | no | Start date |
| end | string | no | End date |
| showInBasket | boolean | no | Show in basket |
| emoji | string | no | Emoji |
| color | string | no | Color |
| externalId | string | no | External ID |
| modificatedDate | string | no | Modification date |
| reviewValidationDate | string | no | Review date |
| reviewValidationInterval | number | no | Review interval enum |
| parent | string | no | Parent project ID |
| parentOrder | number | no | Order in parent |
| journalDate | string | no | Journal date |
| sharedState | object | no | Shared state |
| isNotebook | boolean | no | Is notebook |
| tags | string[] | no | Tag IDs |
| modificated | object | no | Modification details |

### ProjectCreateDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| title | string | **yes** | Project title |
| note | string | no | Note (delta format) |
| start | string | no | Start date |
| end | string | no | End date |
| deleteDate | string | no | Deletion date |
| showInBasket | boolean | no | Show in basket |
| emoji | string | no | Emoji |
| color | string | no | Color |
| externalId | string | no | External ID |
| reviewValidationDate | string | no | Review validation date |
| reviewValidationInterval | number | no | Enum: 0, 1, 2, 3, 4, -1 |
| parent | string | no | Parent project ID |
| parentOrder | number | no | Order in parent |
| isNotebook | boolean | no | Is notebook |

### ProjectUpdateDto

All fields optional, same as ProjectCreateDto plus `journalDate`.

### ProjectCreateResponseDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| project | ProjectResponseDto | yes | Created project |
| taskGroup | TaskGroupResponseDto | yes | Auto-created task group |

---

### TaskResponseDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | yes | Task ID (T-uuid) |
| externalId | string | yes | External ID |
| title | string | yes | Title |
| note | string | yes | Note |
| priority | number | yes | 0=HIGH, 1=NORMAL, 2=LOW |
| recurrence | object | no | Recurrence config |
| journalDate | string | yes | Journal date |
| complete | number | yes | Completion status |
| completeLast | string | yes | Last completion date |
| state | number | yes | 0=PINNED, 1=UNPINNED |
| checked | number | yes | 0=EMPTY, 1=CHECKED, 2=CANCELLED |
| showInBasket | boolean | yes | Show in basket |
| deleteDate | string | yes | Deletion date |
| projectId | string | yes | Project ID |
| recurrenceGeneratorId | string | yes | Recurrence generator ID |
| start | string | yes | Start date |
| startNotifyReaded | boolean | yes | Start notification read |
| notify | number | yes | Notification time |
| startNotifiesReaded | number[] | yes | Read start notifications |
| notifies | number[] | yes | Notifications |
| useTime | boolean | yes | Use time |
| crypted | string | yes | Encrypted content |
| deferred | boolean | yes | Deferred |
| deadline | string | yes | Deadline |
| deadlineNotifyReaded | boolean | yes | Deadline notification read |
| parent | string | yes | Parent task ID |
| group | string | yes | Group ID (Q-uuid) |
| createdDate | string | yes | Creation date |
| modificatedDate | string | yes | Modification date |
| scheduleOrder | number | yes | Schedule order |
| parentOrder | number | yes | Order in parent |
| integrationItemId | string | yes | Integration item ID |
| seenToday | string | yes | Seen today |
| timeLength | number | yes | Duration (minutes) |
| pomodoroCount | number | yes | Pomodoro count |
| pomodoroTotalTime | number | yes | Total pomodoro time |
| alarmNotify | boolean | yes | Alarm notification |
| isNote | boolean | yes | Is note |
| tags | string[] | yes | Tag IDs |
| removed | boolean | yes | Removed flag |
| modificated | object | yes | Modification details |

### TaskCreateDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| title | string | **yes** | Task title |
| note | string | no | Note (delta format) |
| priority | number | no | 0=HIGH, 1=NORMAL, 2=LOW |
| journalDate | string | no | Journal date |
| complete | number | no | Completion status |
| completeLast | string | no | Last completion date |
| state | number | no | 0=PINNED, 1=UNPINNED |
| checked | number | no | 0=EMPTY, 1=CHECKED, 2=CANCELLED |
| showInBasket | boolean | no | Show in basket |
| projectId | string | no | Project ID (P-uuid) |
| start | string | no | Start date |
| startNotifiesReaded | number[] | no | Read start notifications |
| notifies | number[] | no | Notifications |
| useTime | boolean | no | Use time |
| deferred | boolean | no | Deferred |
| deadline | string | no | Deadline |
| deadlineNotifyReaded | boolean | no | Deadline notification read |
| parent | string | no | Parent task ID (T-uuid) |
| group | string | no | Group ID (Q-uuid) |
| scheduleOrder | number | no | Schedule order |
| parentOrder | number | no | Order in parent |
| timeLength | number | no | Duration (minutes) |
| responsible | string | no | Responsible person |
| isNote | boolean | no | Is note |
| tags | string[] | no | Tag IDs |
| externalId | string | no | External ID |

### TaskUpdateDto

All fields optional, same as TaskCreateDto plus `deleteDate`.

---

### TaskGroupResponseDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | yes | Task group ID (Q-uuid) |
| title | string | yes | Title |
| externalId | string (nullable) | yes | External ID |
| modificatedDate | string | yes | Modification date |
| parent | string (nullable) | yes | Parent project ID (P-uuid) |
| parentOrder | number | yes | Order in parent |
| fake | boolean | yes | Fake element flag |
| removed | boolean | yes | Deletion flag |
| modificated | object | no | Modification details |

### TaskGroupCreateDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| title | string | **yes** | Title |
| parent | string | **yes** | Parent project ID (P-uuid) |
| externalId | string | no | External ID |
| parentOrder | number | no | Order in parent |
| fake | boolean | no | Fake flag (default false) |

### TaskGroupUpdateDto

All fields optional: `title`, `externalId`, `parent`, `parentOrder`, `fake`.

---

### KanbanStatusResponseDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | yes | ID (KS-uuid) |
| modificatedDate | string | yes | Modification date |
| removed | boolean | yes | Deletion flag |
| name | string | yes | Status name |
| projectId | string | yes | Project ID (P-uuid) |
| kanbanOrder | number | yes | Display order |
| numberOfColumns | number | yes | Number of columns |
| modificated | object | no | Modification details |

### KanbanStatusCreateDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| name | string | **yes** | Status name |
| projectId | string | **yes** | Project ID (P-uuid) |
| externalId | string | no | External ID |
| kanbanOrder | number | no | Display order (default 0) |
| numberOfColumns | number | no | Number of columns (default 1) |

### KanbanStatusUpdateDto

All fields optional: `externalId`, `name`, `projectId`, `kanbanOrder`, `numberOfColumns`.

---

### KanbanTaskStatusResponseDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | yes | ID (KTS-uuid) |
| modificatedDate | string | yes | Modification date |
| removed | boolean | yes | Deletion flag |
| taskId | string | yes | Task ID (T-uuid) |
| statusId | string | yes | Kanban status ID (KS-uuid) |
| kanbanOrder | number | yes | Display order in status |
| modificated | object | no | Modification details |

### KanbanTaskStatusCreateDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| taskId | string | **yes** | Task ID (T-uuid) |
| statusId | string | **yes** | Kanban status ID (KS-uuid) |
| externalId | string | no | External ID |
| kanbanOrder | number | no | Display order (default 0) |

### KanbanTaskStatusUpdateDto

All fields optional: `externalId`, `taskId`, `statusId`, `kanbanOrder`.

---

### HabitResponseDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | yes | Habit ID |
| title | string | yes | Title |
| description | string | no | Description |
| color | string | no | Color (see HabitColor enum) |
| order | number | no | Order number |
| status | number | yes | Status (0-3) |
| modificatedDate | string | yes | Modification date |
| modificated | object | no | Modification details |
| removed | boolean | yes | Deletion flag |
| externalId | string | no | External ID |

### HabitCreateDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| title | string | **yes** | Title |
| description | string | no | Description |
| color | string | no | Color (see HabitColor enum) |
| order | number | no | Order number |
| status | number | no | Status 0-3 (default 0) |
| externalId | string | no | External ID |

### HabitUpdateDto

All fields optional: `title`, `description`, `color`, `order`, `status`, `externalId`.

---

### HabitDailyProgressResponseDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | yes | Progress record ID |
| habit | string | yes | Habit ID |
| date | string | yes | Date (ISO 8601) |
| progress | number | yes | Progress status |
| modificatedDate | string | yes | Modification date |
| removed | boolean | yes | Deletion flag |
| modificated | object | no | Modification details |

### HabitDailyProgressCreateDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| habit | string | **yes** | Habit ID |
| date | string | **yes** | Date (ISO 8601) |
| progress | number | **yes** | Progress status |

### HabitDailyProgressUpdateDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| progress | number | no | Progress status |

---

### ChecklistItemResponseDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | yes | Item ID (CH-uuid) |
| title | string | yes | Title |
| parent | string | yes | Parent entity ID |
| done | boolean | yes | Completion status |
| parentOrder | number | yes | Display order within parent |
| modificatedDate | string | yes | Modification date |
| modificated | object | no | Modification details |

### ChecklistItemCreateDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| title | string | **yes** | Title |
| parent | string | **yes** | Parent entity ID |
| done | boolean | no | Completion status (default false) |
| parentOrder | number | no | Display order within parent |

### ChecklistItemUpdateDto

All fields optional: `title`, `done`, `parentOrder`.

---

### TagResponseDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | yes | Tag ID |
| title | string | yes | Title |
| parent | string | no | Parent tag ID |
| order | number | no | Display order |
| modificatedDate | string | yes | Modification date |
| removed | boolean | yes | Deletion flag |
| modificated | object | no | Modification details |

### TagCreateDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| title | string | **yes** | Title |
| parent | string | no | Parent tag ID |
| order | number | no | Display order |

### TagUpdateDto

All fields optional: `title`, `parent`, `order`.

---

### TimeStatResponseDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | yes | Time stat ID |
| date | string | yes | Date |
| duration | number | yes | Duration (minutes) |
| relatedTaskId | string | yes | Related task ID |
| modificatedDate | string | yes | Modification date |
| modificated | object | no | Modification details |

### TimeStatCreateDto

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| date | string | **yes** | Date |
| duration | number | **yes** | Duration (minutes) |
| relatedTaskId | string | **yes** | Related task ID |

### TimeStatUpdateDto

All fields optional: `date`, `duration`, `relatedTaskId`.

### TimeStatDeleteBulkResponseDto

| Field | Type | Description |
|-------|------|-------------|
| count | number | Number of deleted records |
