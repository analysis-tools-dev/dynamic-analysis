<!-- 🚨🚨 DON'T EDIT THIS FILE DIRECTLY. Edit `data/tools/` instead. 🚨🚨 -->

 <a href="http://analysis-tools.dev/">
   <img width="400px" alt="Analysis Tools" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/logo.png" />
 </a>

This repository lists **dynamic analysis tools** for all programming languages, build tools, config files and more.  
The official website, [analysis-tools.dev](https://analysis-tools.dev/) is based on this repository and adds rankings and user comments for each tool.

## What is Dynamic Analysis?

> Dynamic program analysis is the analysis of computer software that is performed by executing programs on a real or virtual processor. — [Wikipedia](https://en.wikipedia.org/wiki/Dynamic_program_analysis)
## Sponsors

This project would not be possible without the generous support of our sponsors.

<table>
  <tr>
    <td><a href="https://deepcode.ai"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/deepcode.svg" /></a></td>
    <td><a href="https://deepsource.io/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/deepsource.png" /></a></td>
    <td><a href="https://www.viva64.com/free-license"><img height="100px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/pvs-studio.svg" /></a></td>
    <td><a href="https://codescene.io/"><img width="200px" src="https://raw.githubusercontent.com/analysis-tools-dev/website/master/static/sponsors/codescene.png" /></a></td>
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
      <li><a href="#{{ language.tag }}">{{ language.name }}</a></li>
    {% endfor -%}
  </ul>
</details>

#### [Multiple languages](#multiple-languages-1)

#### [Other](#other-1)

{% for (tag, _) in others -%}
- [{{ tag.name }}](#{{ tag.tag }})
  {% endfor %}

---

## Programming Languages

{%- for (language, linters) in linters %}

<h2 id="{{ language.tag }}">{{ language.name }}</h2>

{% for linter in linters -%}
- [{{linter.name }}]({{linter.homepage }}){% if linter.discussion.is_some() %} [:information_source:](<{{linter.discussion.as_ref().unwrap()}}>){% endif %}{% if linter.deprecated.is_some() %} :warning:{% endif %}{% if linter.license == "proprietary" %} :copyright:{% endif %} - {{ linter.description }}
{% endfor %}

{%- endfor %}

## Multiple languages

{% for linter in multi -%}
- [{{linter.name }}]({{linter.homepage }}){% if linter.discussion.is_some() %} [:information_source:](<{{linter.discussion.as_ref().unwrap()}}>){% endif %}{% if linter.deprecated.is_some() %} :warning:{% endif %}{% if linter.license == "proprietary" %} :copyright:{% endif %} - {{ linter.description }}
{% endfor %}

## Other

{% for (tag, others) in others %}

<h2 id="{{ tag.tag }}">{{ tag.name }}</h2>

{% for other in others -%}
- [{{ other.name }}]({{ other.homepage }}){% if other.discussion.is_some() %} [:information_source:](<{{other.discussion.as_ref().unwrap()}}>){% endif %}{% if other.deprecated.is_some() %} :warning:{% endif %}{% if other.license == "proprietary" %} :copyright:{% endif %} - {{ other.description }}
{% endfor %}

{%- endfor %}

## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
Title image [Designed by Freepik](http://www.freepik.com).
