# Query Reference

This Doc serves as a reference of the syntax and behaviour of queries.


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


## Operators
Only a handfull of operators are supported. However combining them in various ways allows for powerful and versitile queries.


### GET
<code>GET object object_id</code>

#### GET Description
Returns metadata for an [object](#objects).

#### Syntax



### REMOVE
<code>REMOVE object MODE removal_mode</code>

#### REMOVE Description
Removes an [object](#objects).

#### Syntax


### ADD
<code>ADD object object_name object_info</code>

#### ADD Description
Adds a new [object](#objects).

#### Syntax


### UPDATE 
<code>UPDATE object object_id update_operations</code>

#### UPDATE Description
Updates an existing [object](#objects).

#### Syntax


### MATCH 
<code>MATCH pattern WHERE conditions RETURN results</code>

#### MATCH Description and Pattern Matching
Perhabs the most complicated of all operators. It allows for the matching of a pattern against the graph. 
This can be thought of as a canvas of large geometric forms (the graph represented by the database) and a smaller geometric form (the pattern specified in the MATCH statement). What the MATCH operator does is, that it takes the smaller geometric form and searches where it matches (hence the operators name) the existing forms on the canvas. Thereafter the results are filtered and aggregated before they are returned.

#### Syntax
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
