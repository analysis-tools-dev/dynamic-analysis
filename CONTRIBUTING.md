# Thank you for contributing

We welcome pull requests for dynamic analysis tools that meet the requirements
below. **Please verify all criteria before submitting a tool.** If a tool does
not qualify yet, wait until it does rather than opening a pull request or issue.

### Requirements

Before submitting, each tool must

- have existed for at least six months
- have at least 20 stars on GitHub
- have more than one human contributor

Bot and automation accounts do not count toward the contributor minimum. The
check excludes GitHub bot accounts, logins ending in `[bot]`, and known automation
accounts such as `claude`, `dependabot`, and `renovate-bot`, even when GitHub lists
them as ordinary users.

These requirements apply to all tools. Meeting the minimum criteria does not
guarantee inclusion.

For new tool entries, the PR checker uses GitHub repository metadata to check
stars, human contributors, and age. Age is measured by subtracting six calendar
months from the check date, not by counting 180 days. Verified unmet criteria
fail the check, but the checker does not automatically close pull requests.
Missing or unavailable metadata, non-GitHub sources, and tools without a source
URL require manual review. Skipped checks do not count as passes; please provide
evidence of age, usage, and human maintenance for reviewers.

### Format

**The main `README.md` is generated from the data. Do not edit it manually.**
Leave generated `README.md` changes out of your pull request, even if you run
`make render` locally. For changes to the README text or structure, edit
`ci/render/templates/README.md` instead.

To add a new tool, create `data/tools/<toolname>.yml`. Check existing entries for
the required fields and format.

- Use a nonblank tool name of at most **50 UTF-8 bytes** (non-ASCII characters
  can take more than one byte).
- Make each tool description as precise as possible.  
  Please limit the description to **500 characters**.
- Add a license. If it is a proprietary tool, use `license: proprietary`.
- Add at least one tag and as many relevant tags as possible from
  `data/tags.yml`. If a tool needs a new tag, also add it to `data/tags.yml`.

Finally, create a pull request with all your changes.
You can call `make render` to check for errors before.  
This is optional, because it will also be done when creating
a pull request.

### How to mark a tool as unmaintained/deprecated

Sometimes a tool becomes unmaintained and there's nothing wrong with that.  
After all, a tool can still be very valuable to the community - even without
frequent updates.  
However, since it is one of the goals of this project to allow people to make an
informed decision on what is the best tool for the job, we are marking
unmaintained or deprecated tools.
[Here](https://github.com/mre/awesome-static-analysis/issues/223) is a nice
discussion about why we think this is necessary. If you find a tool, which is
unmaintained, please add `deprecated: true` to the entry in `data/tools/` and
create a pull request in which you provide an objective explanation as to why
you think the tool should be marked deprecated. Every deprecation will be
handled on a case-by-case basis.

**Thanks for helping out!** :tada:
