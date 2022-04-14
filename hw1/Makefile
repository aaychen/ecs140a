all: match lcp

lcp: lcp.cpp
	g++ -Wall -Werror -Wextra -o lcp lcp.cpp

match: matching_parentheses.cpp
	g++ -Wall -Werror -Wextra -o match matching_parentheses.cpp
clean:
	rm -f match lcp
