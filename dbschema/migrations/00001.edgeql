CREATE MIGRATION m1pxpvelvlavsjkzrwtcfw7norp3frabshswjzsqcgxmock3m5lk5q
    ONTO initial
{
  CREATE FUTURE nonrecursive_access_policies;
  CREATE TYPE default::Person {
      CREATE REQUIRED PROPERTY name: std::str;
  };
  CREATE TYPE default::Movie {
      CREATE MULTI LINK actors: default::Person;
      CREATE PROPERTY title: std::str;
  };
};
