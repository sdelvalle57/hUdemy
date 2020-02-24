import gql from 'graphql-tag';

export const GET_COURSES = gql`
  query GetCourses($filter: String!) {
    courses(filter: $filter) {
      id
      title
      teacher_address
      students
    }
  }
`;
