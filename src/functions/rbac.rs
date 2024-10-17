use crate::models::rbac::{Role, Permission, RolePermission, UserRole};
use leptos::*;
use spin_sdk::pg::{Connection, ParameterValue};
use spin_sdk::variables;

#[server(CreateRole, "/api")]
pub async fn create_role(name: String, description: Option<String>) -> Result<Role, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO roles (name, description) VALUES ($1, $2) RETURNING id, name, description";
    let params = &[
        ParameterValue::Str(name),
        ParameterValue::Str(description.unwrap_or_default()),
    ];

    let row = conn.query_one(sql, params)?;
    Ok(Role {
        id: row[0].as_i64().unwrap(),
        name: row[1].as_string().unwrap().to_string(),
        description: row[2].as_string().map(|s| s.to_string()),
    })
}

#[server(AssignRoleToUser, "/api")]
pub async fn assign_role_to_user(user_id: i64, role_id: i64) -> Result<(), ServerFnError> {

  }
      Ok(())

      }
;(sql, params)?
  conn.execute];
id),Int64(role_              ParameterValue::
_id),::Int64(user              ParameterValue
 params = &[          let
 NOTHING"; ON CONFLICT DO1, $2)_id) VALUES ($user_id, role INTO user_roles (          let sql = "INSERT
_ids { role_id in role      for

db_url)?; = Connection::open(&      let conn
();_url").unwrap variables::get("db      let db_url =
Error> {<(), ServerFn<i64>) -> Result_ids: Vecd: i64, roleuser(user_i_roles_to_  pub async fn assign
")]esToUser, "/apiRol[server(Assign  #

    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)";
    let params = &[
        ParameterValue::Int64(user_id),
        ParameterValue::Int64(role_id),
    ];

    conn.execute(sql, params)?;
    Ok(())
}

#[server(GetUserRoles, "/api")]
pub async fn get_user_roles(user_id: i64) -> Result<Vec<Role>, ServerFnError> {

  #[server(CheckUserPermission, "/api")]
  pub async fn check_user_permission(user_id: i64, resource: String, action: String) -> Result<bool, ServerFnError> {
    let db_url = variables::get("db_url").expect("Failed to get database URL");
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT COUNT(*) FROM user_roles ur
               JOIN role_permissions rp ON ur.role_id = rp.role_id
               JOIN permissions p ON rp.permission_id = p.id
               WHERE ur.user_id = $1 AND p.resource = $2 AND p.action = $3";
    let params = &[
      ParameterValue::Int64(user_id),
      ParameterValue::Str(resource),
      ParameterValue::Str(action),
    ];

    let row = conn.query_one(sql, params)?;
    let count: i64 = row[0].as_i64().expect("Failed to get count");

    Ok(count > 0)
  }




    let db_url = variables::get("db_url").expect("Failed to get database URL");
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT r.id, r.name, r.description FROM roles r
               JOIN user_roles ur ON r.id = ur.role_id
               WHERE ur.user_id = $1";
    let params = &[ParameterValue::Int64(user_id)];

    let rows = conn.query(sql, params)?;
    let roles = rows.iter().map(|row| Role {
        id: row[0].as_i64().expect("Failed to get role id"),
        name: row[1].as_string().expect("Failed to get role name").to_string(),
        description: row[2].as_string().map(|s| s.to_string()),
    }).collect();

    Ok(roles)
}
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT r.id, r.name, r.description FROM roles r
               JOIN user_roles ur ON r.id = ur.role_id
               WHERE ur.user_id = $1";
    let params = &[ParameterValue::Int64(user_id)];

    let rows = conn.query(sql, params)?;
    let roles = rows.iter().map(|row| Role {
        id: row[0].as_i64().unwrap(),
        name: row[1].as_string().unwrap().to_string(),
        description: row[2].as_string().map(|s| s.to_string()),
    }).collect();

    Ok(roles)
}

// Add more functions for managing permissions and role-permission associations
