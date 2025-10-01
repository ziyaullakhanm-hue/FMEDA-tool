'use strict';
const {
  Model
} = require('sequelize');
module.exports = (sequelize, DataTypes) => {
  class Project extends Model {
    /**
     * Helper method for defining associations.
     * This method is not a part of Sequelize lifecycle.
     * The `models/index` file will call this method automatically.
     */
    static associate(models) {
      // define association here
    }
  }
  Project.init({
    project_name: DataTypes.STRING,
    description: DataTypes.TEXT,
    asil_level: DataTypes.STRING,
    pred_lib_id: DataTypes.INTEGER,
    failure_mode_lib_id: DataTypes.INTEGER,
    created_at: DataTypes.DATE
  }, {
    sequelize,
    modelName: 'Project',
  });
  return Project;
};