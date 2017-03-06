## Some code snippets in markdown

#### leap year example in rust

```rust
pub fn is_leap_year(year:i32) -> bool {
	if (year % 4 == 0) {
		if ((year % 100 != 0) || (year % 400 == 0)){
			return true;
			}
            
		}
		return false;
}
```
#### same example in python

```python
def is_leap_year(year):
    if (year % 4 == 0):
        if ((year % 100 != 0) or (year % 400 == 0)):
            return True
    return False
```

#### some hello-world examples


```dart
// Hello world in Dart

main() {
   print('Hello world!');
}
```

```typescript
// Hello world in TypeScript

alert('Hello world!');
```

```cs
//Hello World in C#
class HelloWorld
{
    static void Main()
    {
        System.Console.WriteLine("Hello, World!");
    }
}
```

```go
// Hello world in Go

package main
import "fmt"bra
func main() {
 fmt.Printf("Hello World\n")
}
```

```java
import javax.servlet.*;
import javax.servlet.http.*;
import java.io.*;

//
// Hello World Java Servlet
//
public class HelloWorld extends HttpServlet {
public void service(HttpServletRequest request,
HttpServletResponse response)
throws IOException {

response.setContentType("text/html");
PrintWriter out = response.getWriter();

out.println("<html><body>");
out.println("Hello World!");
out.println("</body></html>");
}
}
```

```R
# Hello World in R
cat("Hello world\n")
```

```sql
-- Hello World in T-SQL
PRINT 'Hello World'
```

```xhtml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN"
   "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">
<!-- Hello World in XHTML -->
<html
 xmlns="http://www.w3.org/1999/xhtml" xml:lang="en">
 <head>
   <title>
     Hello World!
   </title>
 </head>
 <body>
   <h1>Hello World!</h1>
 </body>
</html>

```

```brainfuck
Hello World in Brainfuck

++++++++++[>+++++++>++++++++++>+++<<<-]>++.>+.+++++++
..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.
```
**source of this Markdown document**: https://helloworldcollection.github.io/