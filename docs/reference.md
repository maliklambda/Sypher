# Query Syntax Reference

This Doc serves as a reference for the SypherQL syntax and behaviour of queries.

## Contents
- [Introduction](#introduction)
- [Basics of Queries](#basics-of-queries)
- [Syntax Reference](#syntax-reference)
- [Subqueries](#subqueries)

## Introduction

## Basics of Queries 
Every valid query starts with one of the [supported_operators](#operators) and returns a [tree of queries](#subquery-trees).<br>
The building blocks of every query are [objects](#objects), [patterns](#match-description-and-pattern-matching), [conditions](#boolean-expression-trees-bet) and [aggregations](#aggregations). 
Multiple queries may be chained together. They are separated by semicolons <code>;</code>.

### Objects
Objects are distinct from [QueryObjects](#query-objects) which hold information about a single query. 
Rather, objects refer to constructs that are usable in the query, through the use of their associated keywords. 
Every object has a keyword associated with it (usually it is the objects name spelled in all caps). 
The following types of objects exist:
1. Node - <code>NODE</code>
2. Relationship - <code>RELATIONSHIP</code>
3. Properties - <code>PROPERTIES</code>
4. Index - <code>INDEX</code>
5. Constraint - <code>CONSTRAINT</code>


### Aggregations
Aggregate results after a [matched pattern](#match-description-and-pattern-matching). 





## Syntax Reference

Only a handfull of operators are supported. However combining them in various ways allows for powerful and versitile queries.

### GET
<code>GET object object_id</code>

#### GET Description
Returns metadata for an [object](#objects).


### REMOVE
<code>REMOVE object MODE removal_mode</code>

#### REMOVE Description
Removes an [object](#objects).



### ADD
<code>ADD object object_name object_info</code>

#### ADD Description
Adds a new [object](#objects).



### UPDATE 
<code>UPDATE object object_id update_operations</code>

#### UPDATE Description
Updates an existing [object](#objects).



### MATCH 
<code>MATCH pattern WHERE conditions RETURN results</code>

#### MATCH Description and Pattern Matching
Perhabs the most complicated of all operators. It allows for the matching of a pattern against the graph. 
This can be thought of as a canvas of large geometric forms (the graph represented by the database) and a smaller geometric form (the pattern specified in the MATCH statement). What the MATCH operator does is, that it takes the smaller geometric form and searches where it matches (hence the operators name) the existing forms on the canvas. Thereafter the results are filtered and aggregated before they are returned.

<code>pattern</code> is a construct that is made up of <strong>nodes</strong> and <strong>relationships</strong>. 
A <code>pattern</code> must contain at least one node and exactly n-1 relationships with n being the number of nodes in the pattern. 
<br>
<p>
    <strong>Nodes</strong> are denoted with parenthesis <code>()</code> in the following way: 
    <code>(identifier_name:node_type)</code><br>
    <code>idenfier_name</code> introduces a variable that may be used in later parts of the statement. 
    <code>identifier_name</code> must be unique, otherwise the query will not execute.
    This may be elided should no specific <code>identifier_name</code> be needed. 
    In this case, a new <code>identifier_name</code> will still be introduced, it will only get an auto-generated name assigned to it.
    For elision of the <code>identifier_name</code>, the syntax <code>(node_type)</code> is used.<br>
    <code>node_type</code> introduces a restriction on the <code>identifier_name</code>s type, as it can only be of type <code>node_type</code>. 
    This may also be elided, in which case no restriction is set on the node.
    The syntax <code>(identifier_name:)</code> is used for this.<br>
    Both elisions may also be combined: <code>()</code> is a valid node.<br>
</p>
<p>
    <strong>Relationships</strong> have a similar syntax that makes use of square brackets <code>[]</code>:
    <code>[identifier_name:relationship_type]</code><br>
    The same rules for elision of either <code>identifier_name</code> or <code>relationship_type</code> or both apply.<br> 
    A <strong>relationship</strong> must also be specified with a <strong>direction</strong>, that may be in- or outgoing. 
    <strong>Directions</strong> are interpreted from left to right.<br>
    Ingoing relationships are therefore denoted like this: 
        <code>(n1) <- (n2)</code> or with a restricted relationship <code>(n1) <-[r:rel_type]- (n2)</code>.
    <br>
    This reads "The node n1 has an ingoing relationship r of type rel_type from node n2." 
    <br>
    Outgoing relationships are denoted like this: 
        <code>(n1) -> (n2)</code> or, again, with a restricted relationship <code>(n1) -[r:rel_type]-> (n2)</code>. 
    <br>
    This reads "The node n1 has an outgoing relationship r of type rel_type to node n2." 
    <br>
    It should be noted here that not specifying 
            <code>identifier_name</code> or <code>relationship_type</code> 
            but keeping the brackets <code>[]</code> is also valid: 
            <code>(n1) -[]-> (n2)</code>.
    <br>
</p>


## Subqueries
In Sypher, a subquery is initialized with the keyword <code>SUBQ</code>. 
The subquery is placed directly after that in curly brackets <code>{}</code>. 
A valid subquery looks like this:<br>
<code>GET NODE SUBQ{MATCH (p:Person) -[LIKES]-> (f:Food) WHERE f.name = "Pizza" RETURN p.id LIMIT 1}</code><br>
(Return the node of a person that likes Pizza.)

### Recursive subqueries
Subqueries can be nested and are parsed recursively. 
For example, this is a valid query:<br>
<code>GET NODE SUBQ{MATCH (p:Person) -[most_popular_relationship:SUBQ{MATCH (p:Person) -[r:]-> () WHERE p.name = "Edos" RETURN r.type_name SORT BY COUNT(r.type_name) DESC LIMIT 1}]-> (unknown:) RETURN unknown.id}</code><br>
(Return the first node with the most popular outgoing relationship type for a person named Edos.)


### Subquery Trees
Every query returns a Subquery Tree, a tree that holds every subquery with its dependencies.
The root of the tree is always the original query.
If there is no subquery, the tree consists only of the root node. 
Every node holds references to the subqueries it depends on. 
This tree structure is what enables [recursive subqueries](#recursive-subqueries).

The following (invalid) query serves as an example.<br> 
<code>OPERATION root SUBQ{subq 1.1 SUBQ{subq 2.1} SUBQ{subq 2.2}} WHERE SUBQ{subq 1.2 SUBQ{subq 2.3 SUBQ{subq 3.1}}}</code><br>
(Invalid query with nested subqueries to illustrate subquery tree structure.)
It will result in the following tree structure:
```mermaid
graph TD
    SR[root] --> S1.1[subq 1.1]
    SR --> S1.2[subq 1.2]
    S1.1 --> S2.1[subq 2.1]
    S1.1 --> S2.2[subq 2.2]
    S1.2 --> S2.3[subq 2.3]
    S2.3 --> S3.1[subq 3.1]
```
### Tree Traversal
When processed, the tree is traversed levelwise from the bottom up. It will start at the leaf node with the greatest depth ("subq3.1" in the [above example](#subquery-trees).
The leaf nodes are processsed first since they are not dependent on other subqueries to be executed first. 
In contrast, all internal nodes must have at least one subquery that needs to be executed before they are.<br>
Internally, the traversal has the following steps: 
1. Start traversal from the trees root.
2. Traverse the entire tree with breadth-first-search. Save references to all visited nodes in a vector v.
3. Reverse v to get the correct order.<br><br>
The [above example](#subquery-trees) will yield the vector <code>[subq3.1, subq2.3, subq2.2, subq2.1, subq1.2, subq1.1, root]</code>.

### String Replacement
The tree stores the start and end index in the original (or root) query for each subquery. 
This makes it possible for the runtime-query-interpreter-engine to replace the entire subquery-string with a string-representation of the result of its execution.

### Query Objects
Every successfully parsed single query (one node of the query-tree) will return a <code>QueryObject</code> which stores the extracted information about the query.
This includes the [operator](#operators) and data associated with it.


## Boolean Expression Trees (BET)
Also called "Binary Expression Trees" or simply "Condition Trees". 
They are internal tree structures to represent the conditions of a [WHERE](#match) clause. 
Each node in the BET holds a FilterCondition, an expression that evaluates to a boolean-value. 
A condition is applied at runtime to the results of a [matched pattern](#match-description-and-pattern-matching). 
Furthermore, a node optionally holds a reference to following nodes that are connected either through the <code>AND</code> or the <code>OR</code> operator.
(Please note that negated conditions using <code>NOT</code> are currently in the works.)
In the following, the child connected by <code>AND</code> is referred to as the left child.
Conversely, the child connected by <code>OR</code> is called the right child.
```mermaid
graph TD
    R[root] -->|and| C1[condition1]
    R -->|or| C2[condition2]
    C1 -->|and| C3[condition3]
    C1 -->|or| C4[condition4]
    C4 -->|or| C7[condition7]
    C2 -->|and| C5[condition5]
    C2 -->|or| C6[condition6]
```
Conditions are parsed sequentially. 
This means that the expressions <code>WHERE A AND B OR C</code> and <code>WHERE A AND (B OR C)</code> are equivalent.
Similarly, the expressions <code>WHERE A OR B AND C</code> and <code>WHERE A OR (B AND C)</code> are equivalent.

For convenience, the root node is initially set to an empty node with a constant condition that will always evaluate to true. 
However, this node is kept only temporarily and the empty root-node is removed from the tree before the runtime executes it. 
The example above corresponds to the following <code>WHERE</code>-clause: 
<code>
WHERE ((condition1 OR condition4 OR condition7) AND condition3) OR (condition2 AND condition5) OR condition6
</code>
<br>
It should be noted here that nested conditions using <code>connection_group</code> characters <code>(</code> and <code>)</code> is still in the works, as this requires a condition's child to either be an atomic condition or a BET itself to remove all ambiguity. 
As of now, only atomic conditions, such as e.g. <code>WHERE condition1 AND condition2 OR condition3</code>, are supported.


### BET Traversal
A [BET](#boolean-expression-trees-bet) is traversed using a DFS-like algorithm (that also resembles inorder traversal of a binary tree).
The algorithm looks for a node that satisfies the following two conditions:
1. It has no child that is connected to it by <code>AND</code>.
2. The node's condition evaluates to true.
<br>
If a node's condition evaluates to false, then the entire left subtree (and-connected) will be discarded, as the expression can never evaluate to true in that case (at least not through said subtree).
However, the right subtree (or-connected) can still yield a true value that will result in a true expression.
So the traversal, upon encountering a node with a false condition, will continue with the node's right (or-connected) subtree.








