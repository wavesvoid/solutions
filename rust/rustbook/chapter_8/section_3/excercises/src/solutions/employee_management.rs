
use std::collections::HashMap;


type DepartmentCollection = HashMap<Department, Vec<Employee>>;


#[derive(Debug, PartialEq)]
pub struct Company {
    departments: DepartmentCollection,
}

impl Company {
    pub fn new() -> Self {
        Self {
            departments: Default::default(),
        }
    }

    pub fn add_employee(&mut self, employee: &str, department: &str) {
        self.departments
            .entry(Department::with_name(department))
            .or_default()
            .push(Employee::with_name(employee));
    } 
}


#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Department {
    pub name: String,
}

impl Department {
    pub fn with_name(name: &str) -> Self {
        Department {
            name: name.to_owned(),
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Employee {
    pub name: String,
}

impl Employee {
    pub fn with_name(name: &str) -> Self {
        Employee {
            name: name.to_owned(),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    mod test_utils {
        use super::{Department, Company, Employee};

        pub type TestDepartments = Vec<(Department, Vec<Employee>)>;
        
        pub fn check_employees(
            company: &Company,
            expected: TestDepartments,
        ) -> bool
        {
            let departments = &company.departments;
            for (name, expected_names) in expected {
                if let Some(names) = departments.get(&name) {
                    assert_eq!(names, &expected_names);
                    continue;
                }
                return false;
            }
            true
        }
    }


    #[test]


    #[test]
    fn test_add_employee() {
        let mut company = Company::new();
        let expected: test_utils::TestDepartments = vec![
            (
                Department { name: "Engineering".to_owned() },
                vec![
                    Employee { name: "John".to_owned() },
                    Employee { name: "Hannah".to_owned() },
                ],
            ),
            (
                Department { name: "DevOps".to_owned() },
                vec![
                    Employee { name: "Alex".to_owned() },
                ],
            ),
        ];

        company.add_employee("John", "Engineering");
        company.add_employee("Hannah", "Engineering");
        company.add_employee("Alex", "DevOps");

        assert!(test_utils::check_employees(&company, expected));
    }
}