<!-- 🚨🚨 DON'T EDIT THIS FILE DIRECTLY. Edit files in `data/tools/` instead. 🚨🚨 -->

<a href="https://analysis-tools.dev/">
  <img alt="Analysis Tools Website" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/redesign.svg" />
</a>

This repository lists **dynamic analysis tools** for finding problems in running programs. It includes profilers, fuzzers, runtime analysis tools, and security testing tools.
The official website, [analysis-tools.dev](https://analysis-tools.dev/) is based on this repository and adds rankings, user comments, and additional resources like videos for each tool.

[![Website](https://img.shields.io/badge/Website-Online-2B5BAE)](https://analysis-tools.dev)
[![CI](https://github.com/analysis-tools-dev/dynamic-analysis/workflows/CI/badge.svg)](https://github.com/analysis-tools-dev/dynamic-analysis/actions/workflows/ci.yml)
[![Links](https://github.com/analysis-tools-dev/dynamic-analysis/actions/workflows/links.yml/badge.svg)](https://github.com/analysis-tools-dev/dynamic-analysis/actions/workflows/links.yml)

## Sponsors

Thank you to CodeRabbit for sponsoring this project, and to everyone who has supported it over the years.

<a href="https://coderabbit.ai">
  <img width="200px" alt="CodeRabbit" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/code-rabbit.svg" />
</a>

Support this project through [GitHub Sponsors](https://github.com/sponsors/analysis-tools-dev) or [Open Collective](https://opencollective.com/analysis-tools).

## Meaning of Symbols:

- :copyright: stands for proprietary software. All other tools are Open Source.
- :information_source: indicates that the community does not recommend to use this tool for new projects anymore. The icon links to the discussion issue.
- :warning: means that this tool was not updated for more than 1 year, or the repo was archived.

Pull requests are very welcome!  
Also check out the sister project, [awesome-static-analysis](https://github.com/mre/awesome-static-analysis).

## Table of Contents

#### [Programming Languages](#programming-languages-1)

| | | |
|---|---|---|
{%- for row in self.linter_rows() %}
|{% for language in row %} [{{ language.name }}](#{{ language.value }}) |{% endfor %}
{%- endfor %}

#### [Multiple languages](#multiple-languages-1)

#### [Other](#other-1)

<details>
<summary>Show Other</summary>

| | | |
|---|---|---|
{%- for row in self.other_rows() %}
|{% for tag in row %} [{{ tag.name }}](#{{ tag.value }}) |{% endfor %}
{%- endfor %}

</details>

---

## Programming Languages

{%- for (language, linters) in linters %}

<a id="{{ language.value }}"></a>
<h2>{{ language.name }}</h2>

{% call tool_list(linters) %}

{%- endfor %}

## Multiple languages

{% call tool_list(multi) %}

## Other

{% for (tag, others) in others %}

<a id="{{ tag.value }}"></a>
<h2>{{ tag.name }}</h2>

{% call tool_list(others) %}

{%- endfor %}

## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
The underlying source code used to format and display that content is licensed under the MIT license.

Title image [Designed by Freepik](https://www.freepik.com/).

{% macro tool(tool) -%}
- [{{ tool.name }}]({{ tool.homepage }}){% if let Some(discussion) = tool.discussion %} [:information_source:](<{{ discussion }}>){% endif %}{% if tool.is_deprecated() %} :warning:{% endif %}{% if tool.license == "proprietary" %} :copyright:{% endif %} — {{ tool.description }}
{%- endmacro %}

{% macro tool_list(entries) -%}
{% for tool in self.active(entries) %}
{% call tool(tool) %}
{% endfor %}
{% for tool in self.deprecated(entries) %}
{% if loop.first %}
<details>
<summary>Show Deprecated</summary>

{% endif %}
{% call tool(tool) %}
{% if loop.last %}

</details>
{% endif %}
{% endfor %}
{%- endmacro %}
