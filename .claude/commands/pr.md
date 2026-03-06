Create pull request. Argument $ARGUMENTS: `[title]`

Steps:
1. Check state: `git status`, `git log origin/main..HEAD`
2. Push if needed: `git push -u origin HEAD`
3. Draft PR title + body (Summary, Changes, Test Plan)
4. `gh pr create --title "..." --body "..."`
5. Report PR URL