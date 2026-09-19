{{ marker }}
## Contributing criteria check

{% if reports.is_empty() %}
No new tool files detected in `data/tools/`. Nothing to check.
{% else %}
{% for report in reports %}
### [{{ report.status() }}] `{{ report.name }}`

{% if let Some(src) = report.source.as_ref() %}
Source: {{ src }}

{% endif %}
{% if let Some(note) = report.note.as_ref() %}
> **Note:** {{ note }}

{% endif %}
| Criterion | Result |
|---|---|
| Stars (min 20) | {{ report.stars.symbol() }} {{ report.stars.message() }} |
| Human contributors (min 2, excluding bots and automation) | {{ report.contributors.symbol() }} {{ report.contributors.message() }} |
| Age (min 6 calendar months) | {{ report.age.symbol() }} {{ report.age.message() }} |

{% endfor %}
---

{% if any_failures %}
One or more tools do not meet the [contributing criteria](CONTRIBUTING.md) yet. The check fails but does not automatically close this PR. Feel free to update it once the thresholds are met.
{% else %}
{% if !any_incomplete %}
All criteria passed. Thank you for your contribution.
{% endif %}
{% endif %}
{% if any_incomplete %}
Manual review required: one or more criteria could not be verified automatically. Please provide evidence for the skipped checks. Skipped checks are not passes.
{% endif %}
{% endif %}