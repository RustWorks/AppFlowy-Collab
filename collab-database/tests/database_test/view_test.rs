use crate::helper::{
  create_database, create_database_grid_view, create_database_with_default_data,
};
use assert_json_diff::assert_json_eq;
use collab::preclude::lib0Any;
use collab_database::fields::Field;
use collab_database::rows::Row;
use collab_database::views::{
  CreateViewParams, Filter, Group, GroupItem, Layout, LayoutSetting, LayoutSettings,
};
use nanoid::nanoid;

use serde_json::json;

#[test]
fn create_initial_database_test() {
  let database_test = create_database(1, "1");
  assert_json_eq!(
    json!({
      "fields": [],
      "rows": [],
      "views": []
    }),
    database_test.to_json_value()
  );
}

#[test]
fn create_database_with_single_view_test() {
  let database_test = create_database_with_default_data(1, "1");
  let params = CreateViewParams {
    view_id: "v1".to_string(),
    name: "my first grid".to_string(),
    layout: Layout::Grid,
    ..Default::default()
  };

  database_test.create_view(params);
  let view = database_test.views.get_view("v1").unwrap();
  assert_eq!(view.row_orders.len(), 3);
  assert_eq!(view.field_orders.len(), 3);
}

#[test]
fn create_same_database_view_twice_test() {
  let database_test = create_database_with_default_data(1, "1");
  let params = CreateViewParams {
    view_id: "v1".to_string(),
    name: "my first grid".to_string(),
    layout: Layout::Grid,
    ..Default::default()
  };
  database_test.create_view(params);

  let params = CreateViewParams {
    view_id: "v1".to_string(),
    name: "my second grid".to_string(),
    layout: Layout::Grid,
    ..Default::default()
  };
  database_test.create_view(params);
  let view = database_test.views.get_view("v1").unwrap();

  assert_eq!(view.name, "my second grid");
}

#[test]
fn create_database_row_test() {
  let database_test = create_database_grid_view(1, "1", "v1");

  let row_id = nanoid!(4);
  database_test.push_row(Row {
    id: row_id.clone(),
    ..Default::default()
  });

  let view = database_test.views.get_view("v1").unwrap();
  assert_json_eq!(view.row_orders.last().unwrap().id, row_id);
}

#[test]
fn create_database_field_test() {
  let database_test = create_database_grid_view(1, "1", "v1");

  let field_id = nanoid!(4);
  database_test.insert_field(Field {
    id: field_id.clone(),
    name: "my third field".to_string(),
    ..Default::default()
  });

  let view = database_test.views.get_view("v1").unwrap();
  assert_json_eq!(view.field_orders.last().unwrap().id, field_id);
}

#[test]
fn create_database_view_with_filter_test() {
  let database_test = create_database_with_default_data(1, "1");
  let filter_1 = Filter {
    id: "filter1".to_string(),
    field_id: "".to_string(),
    field_type: Default::default(),
    condition: 0,
    content: "".to_string(),
  };

  let filter_2 = Filter {
    id: "filter2".to_string(),
    field_id: "".to_string(),
    field_type: Default::default(),
    condition: 0,
    content: "".to_string(),
  };

  let params = CreateViewParams {
    view_id: "v1".to_string(),
    name: "my first grid".to_string(),
    filters: vec![filter_1, filter_2],
    layout: Layout::Grid,
    ..Default::default()
  };
  database_test.create_view(params);

  let view = database_test.views.get_view("v1").unwrap();
  assert_eq!(view.filters.len(), 2);
  assert_eq!(view.filters[0].id, "filter1");
  assert_eq!(view.filters[1].id, "filter2");
}

#[test]
fn create_database_view_with_group_test() {
  let database_test = create_database_with_default_data(1, "1");
  let group_1 = Group {
    id: "group1".to_string(),
    field_id: "".to_string(),
    field_type: Default::default(),
    items: vec![
      GroupItem {
        id: "group_item1".to_string(),
        name: "group item 1".to_string(),
        visible: false,
      },
      GroupItem {
        id: "group_item2".to_string(),
        name: "group item 2".to_string(),
        visible: false,
      },
    ],
    content: "".to_string(),
  };
  let group_2 = Group {
    id: "group2".to_string(),
    field_id: "".to_string(),
    field_type: Default::default(),
    items: vec![],
    content: "".to_string(),
  };

  let params = CreateViewParams {
    view_id: "v1".to_string(),
    groups: vec![group_1, group_2],
    layout: Layout::Grid,
    ..Default::default()
  };
  database_test.create_view(params);

  let view = database_test.views.get_view("v1").unwrap();
  assert_eq!(view.groups.len(), 2);
  assert_eq!(view.groups[0].id, "group1");
  assert_eq!(view.groups[0].items.len(), 2);
  assert_eq!(view.groups[0].items[0].id, "group_item1");
  assert_eq!(view.groups[0].items[1].id, "group_item2");
  assert_eq!(view.groups[1].id, "group2");
}

#[test]
fn create_database_view_with_layout_setting_test() {
  let database_test = create_database_with_default_data(1, "1");
  let mut layout_settings = LayoutSettings::new();
  let mut grid_setting = LayoutSetting::new();
  grid_setting.insert_any("1", lib0Any::BigInt(123));
  grid_setting.insert_any("2", "abc");
  layout_settings.insert(Layout::Grid, grid_setting);

  let params = CreateViewParams {
    view_id: "v1".to_string(),
    layout: Layout::Grid,
    layout_settings,
    ..Default::default()
  };
  database_test.create_view(params);

  let view = database_test.views.get_view("v1").unwrap();
  let grid_layout_setting = view.layout_settings.get(&Layout::Grid).unwrap();
  assert_eq!(grid_layout_setting.get("1").unwrap(), &lib0Any::BigInt(123));
  assert_eq!(
    grid_layout_setting.get("2").unwrap(),
    &lib0Any::String("abc".to_string().into_boxed_str())
  );
}

#[test]
fn delete_inline_database_view_test() {
  let database_test = create_database_with_default_data(1, "1");
  for i in 0..3 {
    let params = CreateViewParams {
      view_id: format!("v{}", i),
      ..Default::default()
    };
    database_test.create_view(params);
  }

  let views = database_test.views.get_all_views();
  let view_id = views[1].id.clone();
  assert_eq!(views.len(), 3);

  database_test.views.delete_view(&view_id);
  let views = database_test
    .views
    .get_all_views()
    .iter()
    .map(|view| view.id.clone())
    .collect::<Vec<String>>();
  assert_eq!(views.len(), 2);
  assert!(!views.contains(&view_id));
}

#[test]
fn duplicate_database_view_test() {
  let database_test = create_database_with_default_data(1, "1");
  let params = CreateViewParams {
    view_id: "v1".to_string(),
    ..Default::default()
  };
  database_test.create_view(params);
  database_test.duplicate_view("v1");

  let views = database_test.views.get_all_views();
  assert_eq!(views.len(), 2);
}