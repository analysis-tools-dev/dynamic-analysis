<!-- 🚨🚨 DON'T EDIT THIS FILE DIRECTLY. Edit `data/tools.yml` instead. 🚨🚨 -->

<a href="https://analysis-tools.dev/">
  <img alt="Analysis Tools Website" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/redesign.svg" />
</a>

This repository lists **dynamic analysis tools** for all programming languages, build tools, config files and more. The focus is on tools which improve code quality such as linters and formatters.
The official website, [analysis-tools.dev](https://analysis-tools.dev/) is based on this repository and adds rankings, user comments, and additional resources like videos for each tool.

[![Website](https://img.shields.io/badge/Website-Online-2B5BAE)](https://analysis-tools.dev)
![CI](https://github.com/analysis-tools-dev/dynamic-analysis/workflows/CI/badge.svg)

## Sponsors

This project would not be possible without the generous support of our sponsors.

<table>
   <tr>
      <td><a href="https://deepcode.ai"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/deepcode.png" /></a></td>
      <td>
         <a href="https://www.bearer.com">
            <picture >
               <source width="200px" media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/bearer-dark.svg">
               <img width="200px" alt="Bearer" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/bearer-light.svg">
            </picture>
         </a>
      </td>
      <td><a href="https://codescene.io/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/codescene.svg" /></a></td>
      <td><a href="https://semgrep.dev/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/semgrep.svg" /></a></td>
      <td><a href="https://codiga.io/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/codiga.svg" /></a></td>
      <td><a href="https://offensive360.com/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/assets/master/static/sponsors/offensive360.png" /></a></td>
   </tr>
</table>

If you also want to support this project, head over to our [Github sponsors page](https://github.com/sponsors/analysis-tools-dev).

## Meaning of Symbols:

- :copyright: stands for proprietary software. All other tools are Open Source.
- :information_source: indicates that the community does not recommend to use this tool for new projects anymore. The icon links to the discussion issue.
- :warning: means that this tool was not updated for more than 1 year, or the repo was archived.

Pull requests are very welcome!  
Also check out the sister project, [awesome-static-analysis](https://github.com/mre/awesome-static-analysis).

## Table of Contents

#### [Programming Languages](#programming-languages-1)

<details>
 <summary>Show languages</summary>
  <!-- Please use HTML syntax here so that it works for Github and mkdocs -->
  <ul>
    {% for (language, _) in linters -%}
      <li><a href="#{{ language.value }}">{{ language.name }}</a></li>
    {% endfor -%}
  </ul>
</details>

#### [Multiple languages](#multiple-languages-1)

#### [Other](#other-1)

{% for (tag, _) in others %}

- [{{ tag.name }}](#{{ tag.value }})
  {% endfor %}

---

## Programming Languages

{%- for (language, linters) in linters %}

<h2 id="{{ language.value }}">{{ language.name }}</h2>

{% for linter in linters %}

- [{{linter.name }}]({{linter.homepage }}){% if linter.discussion.is_some() %} [:information_source:](<{{linter.discussion.as_ref().unwrap()}}>){% endif %}{% if linter.deprecated.is_some() %} :warning:{% endif %}{% if linter.license == "proprietary" %} :copyright:{% endif %} — {{ linter.description }}
  {% endfor %}

{%- endfor %}

## Multiple languages

{% for linter in multi %}

- [{{linter.name }}]({{linter.homepage }}){% if linter.discussion.is_some() %} [:information_source:](<{{linter.discussion.as_ref().unwrap()}}>){% endif %}{% if linter.deprecated.is_some() %} :warning:{% endif %}{% if linter.license == "proprietary" %} :copyright:{% endif %} — {{ linter.description }}
  {% endfor %}

## Other

{% for (tag, others) in others %}

<h2 id="{{ tag.value }}">{{ tag.name }}</h2>

{% for other in others %}

- [{{ other.name }}]({{ other.homepage }}){% if other.discussion.is_some() %} [:information_source:](<{{other.discussion.as_ref().unwrap()}}>){% endif %}{% if other.deprecated.is_some() %} :warning:{% endif %}{% if other.license == "proprietary" %} :copyright:{% endif %} — {{ other.description }}
  {% endfor %}

{%- endfor %}

## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
The underlying source code used to format and display that content is licensed under the MIT license.

Title image [Designed by Freepik](http://www.freepik.com).
