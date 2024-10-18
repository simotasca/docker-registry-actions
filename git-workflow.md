for versioning
- list branches: `git branch -l`
- create branch `git branch <branch-name>`
- switch current branch: `git switch <branch-name>`
- push the branch to github: `git push origin <branch-name>`
- back to master: `git switch master`

for merging:
- create branch `git branch <feature/branch-name>`
- switch current branch: `git switch <feature/branch-name>`
- push the branch to github: `git push origin <feature/branch-name>`
- push modifications on the branch: `git push origin <feature/branch-name>`
- back to master: `git switch master`
- pull master: `git pull origin master`
- merge: `git merge <feature/branch-name>`
- delete local branch: `git branch -d <feature/branch-name>`
- delete remote branch: `git push origin --delete <feature/branch-name>`