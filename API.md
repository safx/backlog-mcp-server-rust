# Backlog API Implementation Status

## Summary
- Total APIs listed: 159
- Implemented: 152 (96%)
- Not implemented: 7 (4%)

### Spaces
- ✅ GetSpace: Returns information about your space: GET /api/v2/space
- ✅ GetSpaceLogo: Returns logo image of your space: GET /api/v2/space/image
- ✅ GetLicence: Returns licence: GET /api/v2/space/licence
- ✅ GetSpaceDiskUsage: Returns information about space disk usage: GET /api/v2/space/diskUsage
- ✅ GetSpaceNotification: Returns space notification: GET /api/v2/space/notification
- ✅ UpdateSpaceNotification: Updates space notification: PUT /api/v2/space/notification
- ✅ PostAttachmentFile: Posts an attachment file for issue or wiki. Returns id of the attachment file: POST /api/v2/space/attachment

### Projects
- ✅ GetProject: Returns information about project: GET /api/v2/projects/:projectIdOrKey
- ✅ GetProjectList: Returns list of projects: GET /api/v2/projects
- ✅ GetProjectIcon: Downloads project icon: GET /api/v2/projects/:projectIdOrKey/image
- ✅ AddProject: Adds new project: POST /api/v2/projects
- ✅ DeleteProject: Deletes project: DELETE /api/v2/projects/:projectIdOrKey
- ✅ GetProjectDiskUsage: Returns information about project disk usage: GET /api/v2/projects/:projectIdOrKey/diskUsage
- ✅ UpdateProject: Updates information about project: PATCH /api/v2/projects/:projectIdOrKey

#### Users
- ✅ GetProjectUserList: Returns list of project members: GET /api/v2/projects/:projectIdOrKey/users
- ✅ AddProjectUser: Adds user to list of project members: POST /api/v2/projects/:projectIdOrKey/users
- ✅ DeleteProjectUser: Removes user from list project members: DELETE /api/v2/projects/:projectIdOrKey/users

#### Project Admin
- ✅ GetListOfProjectAdministrators: Returns list of users who has Project Administrator role: GET /api/v2/projects/:projectIdOrKey/administrators
- ✅ AddProjectAdministrator: Adds “Project Administrator” role to user: POST /api/v2/projects/:projectIdOrKey/administrators
- ✅ DeleteProjectAdministrator: Removes Project Administrator role from user: DELETE /api/v2/projects/:projectIdOrKey/administrators

#### Teams
- ✅ AddProjectTeam: Add team to project: POST /api/v2/projects/:projectIdOrKey/teams
- ✅ DeleteProjectTeam: Removes a team from the project: DELETE /api/v2/projects/:projectIdOrKey/teams
- ✅ GetProjectTeamList: Returns list of project teams: GET /api/v2/projects/:projectIdOrKey/teams


### Issues
- ✅ GetIssueList: Returns list of issues: GET /api/v2/issues
- ✅ GetIssue: Returns information about issue: GET /api/v2/issues/:issueIdOrKey
- ✅ AddIssue: Adds new issue: POST /api/v2/issues
- ✅ DeleteIssue: Deletes issue: DELETE /api/v2/issues/:issueIdOrKey
- ✅ CountIssue: Returns number of issues: GET /api/v2/issues/count
- ✅ UpdateIssue: Updates information about issue: PATCH /api/v2/issues/:issueIdOrKey

#### Attachements
- ✅ GetListOfIssueAttachments: Returns the list of issue attachments: GET /api/v2/issues/:issueIdOrKey/attachments
- ✅ GetIssueAttachment: Downloads issue’s attachment file: GET /api/v2/issues/:issueIdOrKey/attachments/:attachmentId
- ✅ DeleteIssueAttachment: Deletes an attachment of issue: DELETE /api/v2/issues/:issueIdOrKey/attachments/:attachmentId

#### Shared files for issue
- ✅ GetListOfLinkedSharedFiles: Returns the list of linked Shared Files to issues: GET /api/v2/issues/:issueIdOrKey/sharedFiles
- ✅ LinkSharedFilesToIssue: Links shared files to issue: POST /api/v2/issues/:issueIdOrKey/sharedFiles
- ✅ RemoveLinkToSharedFileFromIssue: Removes link to shared file from issue: DELETE /api/v2/issues/:issueIdOrKey/sharedFiles/:id

#### Related issues
- ✅ GetListOfRelatedIssues: Returns the list of related issues: GET /api/v2/issues/:issueIdOrKey/relatedIssues
- ✅ AddRelatedIssue: Adds a related issue: POST /api/v2/issues/:issueIdOrKey/relatedIssues
- ✅ RemoveRelatedIssue: Removes a related issue: DELETE /api/v2/issues/:issueIdOrKey/relatedIssues/:relatedIssueId

### Comment
- ✅ UpdateComment: Updates content of comment: PATCH /api/v2/issues/:issueIdOrKey/comments/:commentId
- ✅ CountComment: Returns number of comments in issue: GET /api/v2/issues/:issueIdOrKey/comments/count
- ✅ AddComment: Adds a comment to the issue: POST /api/v2/issues/:issueIdOrKey/comments
- ✅ DeleteComment: Delete comment: DELETE /api/v2/issues/:issueIdOrKey/comments/:commentId
- ✅ GetCommentList: Returns list of comments in issue: GET /api/v2/issues/:issueIdOrKey/comments
- ✅ GetComment: Returns information about comment: GET /api/v2/issues/:issueIdOrKey/comments/:commentId
- ✅ GetIssueParticipantList: Returns list of issue participants: GET /api/v2/issues/:issueIdOrKey/participants
- ✅ GetListOfCommentNotifications: Returns the list of comment notifications: GET /api/v2/issues/:issueIdOrKey/comments/:commentId/notifications
- ✅ AddCommentNotification: Adds notifications to the comment. Only the user who added the comment can add notifications: POST /api/v2/issues/:issueIdOrKey/comments/:commentId/notifications

### Priority
- ✅ GetPriorityList: Returns list of priorities: GET /api/v2/priorities

### Resolution
- ✅ GetResolutionList: Returns list of resolutions: GET /api/v2/resolutions

### Category
- ✅ AddCategory: Adds new Category to the project: POST /api/v2/projects/:projectIdOrKey/categories
- ✅ UpdateCategory: Updates information about Category: PATCH /api/v2/projects/:projectIdOrKey/categories/:id
- ✅ DeleteCategory: Deletes Category: DELETE /api/v2/projects/:projectIdOrKey/categories/:id
- ✅ GetCategoryList: Returns list of Categories in the project: GET /api/v2/projects/:projectIdOrKey/categories

### Custom Fields
- ✅ AddCustomField: Adds new Custom Field to the project: POST /api/v2/projects/:projectIdOrKey/customFields
- ✅ UpdateCustomField: Updates Custom Field: PATCH /api/v2/projects/:projectIdOrKey/customFields/:id
- ✅ GetCustomFieldList: Returns list of Custom Fields in the project: GET /api/v2/projects/:projectIdOrKey/customFields
- ✅ AddListItemForListTypeCustomField: Adds new list item for list type custom field. Only administrator can call this API if the option “Add items in adding or editing issues” is disabled in settings. Calling API fails if specified custom field’s type is not a list: POST /api/v2/projects/:projectIdOrKey/customFields/:id/items
- ✅ DeleteCustomField: Deletes Custom Field: DELETE /api/v2/projects/:projectIdOrKey/customFields/:id
- ✅ DeleteListItemForListTypeCustomField: Deletes list item for list type custom field. Calling API fails if specified custom field’s type is not a list: DELETE /api/v2/projects/:projectIdOrKey/customFields/:id/items/:itemId
- ✅ UpdateListItemForListTypeCustomField: Updates list item for list type custom field. Calling API fails if specified custom field’s type is not a list: PATCH /api/v2/projects/:projectIdOrKey/customFields/:id/items/:itemId

### Issue Type
- ✅ UpdateIssueType: Updates information about Issue Type: PATCH /api/v2/projects/:projectIdOrKey/issueTypes/:id
- ✅ GetIssueTypeList: Returns list of Issue Types in the project: GET /api/v2/projects/:projectIdOrKey/issueTypes
- ✅ AddIssueType: Adds new Issue Type to the project: POST /api/v2/projects/:projectIdOrKey/issueTypes
- ✅ DeleteIssueType: Deletes Issue Type: DELETE /api/v2/projects/:projectIdOrKey/issueTypes/:id

### Status
- ✅ GetStatusListOfProject: Returns list of status in the project: GET /api/v2/projects/:projectIdOrKey/statuses
- ✅ AddStatus: Adds new Status to the project. You can create up to 8 custom statuses within a Project aside from the 4 default: POST /api/v2/projects/:projectIdOrKey/statuses
- ✅ UpdateOrderOfStatus: Updates order about Status: PATCH /api/v2/projects/:projectIdOrKey/statuses/updateDisplayOrder
- ✅ UpdateStatus: Updates information about Status: PATCH /api/v2/projects/:projectIdOrKey/statuses/:id
- ✅ DeleteStatus: Deletes Status: DELETE /api/v2/projects/:projectIdOrKey/statuses/:id

### Version/Milestone
- ✅ GetVersionMilestoneList: Returns list of Versions/Milestones in the project: GET /api/v2/projects/:projectIdOrKey/versions
- ✅ AddVersionMilestone: Adds new Version/Milestone to the project: POST /api/v2/projects/:projectIdOrKey/versions
- ✅ DeleteVersion: Deletes Version: DELETE /api/v2/projects/:projectIdOrKey/versions/:id
- ✅ UpdateVersionMilestone: Updates information about Version/Milestone: PATCH /api/v2/projects/:projectIdOrKey/versions/:id

### Webhook
- ✅ GetListOfWebhooks: Returns list of webhooks: GET /api/v2/projects/:projectIdOrKey/webhooks
- ✅ AddWebhook: Adds new webhook: POST /api/v2/projects/:projectIdOrKey/webhooks
- ✅ DeleteWebhook: Deletes webhook: DELETE /api/v2/projects/:projectIdOrKey/webhooks/:webhookId
- ✅ UpdateWebhook: Updates information about webhook: PATCH /api/v2/projects/:projectIdOrKey/webhooks/:webhookId
- ✅ GetWebhook: Returns information about webhook: GET /api/v2/projects/:projectIdOrKey/webhooks/:webhookId

### Shared Files
- ✅ GetFile: Downloads the file: GET /api/v2/projects/:projectIdOrKey/files/:sharedFileId
- ✅ GetListOfSharedFiles: Gets list of Shared Files: GET /api/v2/projects/:projectIdOrKey/files/metadata/:path

### Wikis
- ✅ UpdateWikiPage: Updates information about Wiki page: PATCH /api/v2/wikis/:wikiId
- ✅ GetWikiPageAttachment: Downloads Wiki page’s attachment file: GET /api/v2/wikis/:wikiId/attachments/:attachmentId
- ✅ GetWikiPageHistory: Returns history of Wiki page: GET /api/v2/wikis/:wikiId/history
- ✅ GetWikiPageList: Returns list of Wiki pages: GET /api/v2/wikis
- ✅ GetWikiPageTagList: Returns list of tags that are used in the project: GET /api/v2/wikis/tags
- ✅ GetWikiPage: Returns information about Wiki page: GET /api/v2/wikis/:wikiId
- ✅ GetListOfWikiAttachments: Gets list of files attached to Wiki: GET /api/v2/wikis/:wikiId/attachments
- ✅ RemoveWikiAttachment: Removes files attached to Wiki: DELETE /api/v2/wikis/:wikiId/attachments/:attachmentId
- ✅ DeleteWikiPage: Deletes Wiki page: DELETE /api/v2/wikis/:wikiId
- ✅ CountWikiPage: Returns number of Wiki pages: GET /api/v2/wikis/count
- ✅ AddWikiPage: Adds new Wiki page: POST /api/v2/wikis
- ✅ AttachFileToWiki: Attaches file to Wiki: POST /api/v2/wikis/:wikiId/attachments

#### Shared files for wiki
- ✅ GetListOfSharedFilesOnWiki: Returns the list of Shared Files on Wiki: GET /api/v2/wikis/:wikiId/sharedFiles
- ✅ LinkSharedFilesToWiki: Links Shared Files to Wiki: POST /api/v2/wikis/:wikiId/sharedFiles
- ✅ RemoveLinkToSharedFileFromWiki: Removes link to shared file from Wiki: DELETE /api/v2/wikis/:wikiId/sharedFiles/:id

### Git
- ✅ GetGitRepository: Returns Git repository: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName
- ✅ GetListOfGitRepositories: Returns list of Git repositories: GET /api/v2/projects/:projectIdOrKey/git/repositories

### Pull Request
- ✅ UpdatePullRequestCommentInformation: Updates pull request comment information: PATCH /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments/:commentId
- ✅ UpdatePullRequest: Updates pull requests: PATCH /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number
- ✅ GetNumberOfPullRequestComments: Returns number of comments on pull requests: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments/count
- ✅ GetNumberOfPullRequests: Returns number of pull requests: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/count
- ✅ GetListOfPullRequestAttachment: Returns list of attached files on pull requests: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/attachments
- ✅ GetPullRequestComment: Returns list of pull request comments: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments
- ✅ GetPullRequestList: Returns list of pull requests: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests
- ✅ GetPullRequest: Returns pull reuqest: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number
- ✅ DownloadPullRequestAttachment: Downloads attached files on pull requests: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/attachments/:attachmentId
- ✅ DeletePullRequestAttachments: Deletes attached files on pull requests: DELETE /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/attachments/:attachmentId
- ✅ AddPullRequestComment: Adds comments on pull requests: POST /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments
- ✅ AddPullRequest: Adds pull requests: POST /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests


### Activities
- ✅ GetActivity: Returns an activity: GET /api/v2/activities/:activityId
- ✅ GetUserRecentUpdates: Returns user’s recent updates: GET /api/v2/users/:userId/activities
- ✅ GetProjectRecentUpdates: Returns recent update in the project: GET /api/v2/projects/:projectIdOrKey/activities
- ✅ GetRecentUpdates: Returns recent updates in your space: GET /api/v2/space/activities

### Notifications
- ✅ CountNotification: Returns number of Notifications: GET /api/v2/notifications/count
- ✅ GetNotification: Returns own notifications: GET /api/v2/notifications
- ✅ ReadNotification: Changes notifications read: POST /api/v2/notifications/:id/markAsRead
- ✅ ResetUnreadNotificationCount: Resets unread Notification count: POST /api/v2/notifications/markAsRead

### Watchings
- ✅ UpdateWatching: Updates a watching. User can update own note: PATCH /api/v2/watchings/:watchingId
- ✅ AddWatching: Adds a watching. User can add a own watching: POST /api/v2/watchings
- ✅ CountWatching: Returns the number of your watching issues: GET /api/v2/users/:userId/watchings/count
- ✅ DeleteWatching: Deletes a own watching. User can delete a own watching: DELETE /api/v2/watchings/:watchingId
- ✅ GetWatchingList: Returns list of your watching issues: GET /api/v2/users/:userId/watchings
- ✅ GetWatching: Returns the information about a watching: GET /api/v2/watchings/:watchingId
- ✅ MarkWatchingAsRead: Mark a watching as read: POST /api/v2/watchings/:watchingId/markAsRead

### Star
- ✅ AddStar: Adds star: POST /api/v2/stars
- ✅ RemoveStar: Removes star: DELETE /api/v2/stars/:starId
- ✅ GetWikiPageStar: Returns list of stars received on the Wiki page: GET /api/v2/wikis/:wikiId/stars
- ✅ CountUserReceivedStars: Returns number of stars that user received: GET /api/v2/users/:userId/stars/count
- ✅ GetReceivedStarList: Returns the list of stars that user received: GET /api/v2/users/:userId/stars

### Document
- ✅ GetDocumentList: Returns list of document pages: GET /api/v2/documents
- ✅ GetDocumentCount: Counts documents in one project: GET /api/v2/documents/count
- ✅ AddDocument: Creates a document: POST /api/v2/documents
- ✅ DeleteDocument: Deletes a document: DELETE /api/v2/documents/:documentId
- ✅ AddDocumentTag: Adds named tags to a document: POST /api/v2/documents/:documentId/tags
- ✅ RemoveDocumentTag: Removes named tags from a document (204 No Content): DELETE /api/v2/documents/:documentId/tags
- ✅ GetDocumentTree: Retrieves the document tree: GET /api/v2/documents/tree
- ✅ GetDocument: Returns information about document page: GET /api/v2/documents/:documentId
- ✅ DownloadDocumentAttachment: Downloads document attachments: GET /api/v2/documents/:documentId/attachments/:attachmentId
- ✅ GetDocumentComments: Returns list of comments on document page: GET /api/v2/documents/:documentId/comments

### Users
- ✅ GetOwnUser: Returns own information about user: GET /api/v2/users/myself
- ✅ GetUserList: Returns list of users in your space: GET /api/v2/users
- ✅ GetUser: Returns information about user: GET /api/v2/users/:userId
- ✅ GetUserIcon: Downloads user icon: GET /api/v2/users/:userId/icon
- ❌ (Classic) AddUser: Adds new user to the space. “Project Administrator” cannot add “Admin” user. You can’t use this API at new plan space: POST /api/v2/users
- ❌ (Classic) UpdateUser: Updates information about user. You can’t use this API at new plan space: PATCH /api/v2/users/:userId
- ❌ (Classic) DeleteUser: Deletes user from the space. You can’t use this API at new plan space: DELETE /api/v2/users/:userId

### Teams
- ✅ GetTeamIcon: Downloads team icon: GET /api/v2/teams/:teamId/icon
- ✅ GetTeam: Returns information about team: GET /api/v2/teams/:teamId
- ✅ GetListOfTeams: Returns list of teams: GET /api/v2/teams
- ❌ (Classic) AddTeam: Adds new team. You can’t use this API at new plan space: POST /api/v2/teams
- ❌ (Classic) UpdateTeam: Updates information about team. You can’t use this API at new plan space: PATCH /api/v2/teams/:teamId
- ❌ (Classic) DeleteTeam: Deletes team. You can’t use this API at new plan space: DELETE /api/v2/teams/:teamId

### Recent
- ✅ AddRecentlyViewedIssue: Add an issue which the user viewed recently: POST /api/v2/users/myself/recentlyViewedIssues
- ✅ GetListOfRecentlyViewedIssues: Returns list of issues which the user viewed recently: GET /api/v2/users/myself/recentlyViewedIssues
- ✅ GetListOfRecentlyViewedProjects: Returns list of projects which the user viewed recently: GET /api/v2/users/myself/recentlyViewedProjects
- ✅ GetListOfRecentlyViewedWikis: Returns list of Wikis which the user viewed recently: GET /api/v2/users/myself/recentlyViewedWikis
- ✅ AddRecentlyViewedWiki: Add a wiki which the user viewed recently: POST /api/v2/users/myself/recentlyViewedWikis

### Rate Limit
- ✅ GetRateLimit: Return information about the rate limit currently applied to you: GET /api/v2/rateLimit

### OAuth 2.0 (Authentication)
- ❌ POST /api/v2/oauth2/token: Issues an access token.
