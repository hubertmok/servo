// -- WARNING -- WARNING -- WARNING -- WARNING -- WARNING -- WARNING -- 
//
// THE CONTENTS OF THIS FILE IS GENERATED BY CODE AND
// ANY MODIFICATIONS WILL BE OVERWRITTEN
//
// -- WARNING -- WARNING -- WARNING -- WARNING -- WARNING -- WARNING -- 

// %BANNER_BEGIN%
// ---------------------------------------------------------------------
// %COPYRIGHT_BEGIN%
//
// Copyright (c) 2018 Magic Leap, Inc. All Rights Reserved.
// Use of this file is governed by the Creator Agreement, located
// here: https://id.magicleap.com/creator-terms
//
// %COPYRIGHT_END%
// ---------------------------------------------------------------------
// %BANNER_END%

#pragma once

#include <string>
#include <map>

// data class 
class SceneDescriptor {
  public:

    typedef std::map<std::string /* exportedNodeName */, const std::string& /* exportedNodeId */> ExportedNodeReferences;

    SceneDescriptor(const char* exportedName, const char* id, const char* sceneGraphFilePath, const char* resourceModelFilePath, const ExportedNodeReferences& exportedNodeReferences, bool initiallyInstanced);
    const std::string& getExportedName() const;
    const std::string& getId() const;
    const std::string& getSceneGraphPath() const;
    const std::string& getResourceModelPath() const;
    const ExportedNodeReferences & getExportedNodeReferences() const;
    bool getInitiallyInstanced() const;

  private:
    std::string exportedName_;
    std::string id_;
    std::string sceneGraphPath_;
    std::string resourceModelPath_;
    const ExportedNodeReferences& exportedNodeReferences_;
    bool initiallyInstanced_;
};

typedef std::map<std::string /* exportedName */, const SceneDescriptor&> SceneDescriptorReferences;
