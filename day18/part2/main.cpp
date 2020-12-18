#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <numeric>
#include <stack>
#include <stdexcept>
#include <string>
#include <vector>

namespace parser
{
namespace
{
int get_operator_priority(char op)
{
    if (op == '(' or op == ')')
    {
        return 0;
    }
    else if (op == '*')
    {
        return 1;
    }
    else
    {
        return 2;
    }
}

void found_operator(char c, std::string& postfix, std::stack<char>& operator_stack)
{
    while ((not operator_stack.empty()) and
        (operator_stack.top() != '(') and
        (get_operator_priority(c) <= get_operator_priority(operator_stack.top())))
    {
        postfix.push_back(operator_stack.top());
        postfix.push_back(' ');
        operator_stack.pop();
    }
    operator_stack.push(c);
}

void found_right_parenthesis(std::string& postfix, std::stack<char>& operator_stack)
{
    while ((not operator_stack.empty()) and (operator_stack.top() != '('))
    {
        postfix.push_back(operator_stack.top());
        operator_stack.pop();
    }
    operator_stack.pop();
}
} // namespace

std::string make_postfix_expression(const std::string& line)
{
    std::string input{};
    std::copy_if(line.begin(), line.end(), std::back_inserter(input), [](const auto c) {
        return c != ' ';
    });

    std::string postfix{};
    std::stack<char> operator_stack{};

    for (auto it = input.begin(); it != input.end(); ++it)
    {
        if (isdigit(*it))
        {
            postfix.push_back(*it);
            postfix.push_back(' ');
        }
        else if (*it != '(' and *it != ')')
        {
            found_operator(*it, postfix, operator_stack);
        }
        else if (*it == '(')
        {
            operator_stack.push(*it);
        }
        else
        {
            found_right_parenthesis(postfix, operator_stack);
        }
    }

    while (not operator_stack.empty())
    {
        postfix.push_back(' ');
        postfix.push_back(operator_stack.top());
        operator_stack.pop();
    }

    return postfix;
}
} // namespace parser

namespace evaluator
{
namespace
{
std::int64_t process(char op, std::int64_t lhs, std::int64_t rhs)
{
    switch (op)
    {
        case '+':
        {
            return lhs + rhs;
        }
        case '*':
        {
            return lhs * rhs;
        }
        default:
        {
            throw std::runtime_error("Unknown operation");
        }
    }
}
} // namespace

std::int64_t evaluate(std::string expression)
{
    std::stack<std::int64_t> operands;

    for (auto it = expression.begin(); it != expression.end(); ++it)
    {
        if (isdigit(*it))
        {
            operands.push(*it - '0');
        }
        else if (*it != ' ')
        {
            const auto rhs = operands.top();
            operands.pop();
            const auto lhs = operands.top();
            operands.pop();
            operands.push(process(*it, lhs, rhs));
        }
    }

    return operands.top();
}
} // namespace evaluator

std::vector<std::string> load_file(const char* filepath)
{
    std::vector<std::string> output{};
    std::ifstream file(filepath);

    std::string line{};
    while (std::getline(file, line))
    {
        output.emplace_back(line);
    }

    return output;
}

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        throw std::runtime_error("Not filename provided");
    }

    const auto input = load_file(argv[1]);

    std::vector<std::int64_t> results{};
    std::transform(input.begin(), input.end(), std::back_inserter(results), [](const auto& line) {
        auto expr = parser::make_postfix_expression(line);
        return evaluator::evaluate(std::move(expr));
    });

    const auto result = std::accumulate(results.begin(), results.end(), std::int64_t{});
    std::cout << result << std::endl;
}