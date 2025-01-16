# CodeCrafters challenge: build your own shell

This is a blog about the [CodeCrafters build your own shell challenge](https://app.codecrafters.io/courses/shell/introduction). I will be tackling this challenge in Rust: most (if not all) of my programming experience has been in Python, and I'm eager to learn a second language. After doing some research into programming languages, their features, popularity, ease of learning, I chose Rust as the language I'd like to dive into and learn. 

## Blog Posts
<ul>
  {% for post in site.posts %}
    <li>
      <a href="{{ site.baseurl }}{{ post.url }}">{{ post.title }}</a> - <small>{{ post.date | date: "%B %d, %Y" }}</small>
    </li>
  {% endfor %}
</ul>
