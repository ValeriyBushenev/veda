// Individual Presenter

veda.Module(function IndividualPresenter(veda) { "use strict";

  var deletedAlertTmpl = $("#deleted-individual-alert-template").html();

  //var c = 0;

  veda.on("individual:loaded", function (individual, container, template, mode) {

    //console.log(individual.id, "presenter count:", ++c);

    if (typeof container === "string") {
      container = $(container).empty();
    }
    mode = mode || "view";

    if (container.prop("id") === "main") { container.hide(); }

    // Change location.hash if individual was presented in #main container
    if (container.prop("id") === "main" && location.hash.indexOf(individual.id) < 0) {
      var hash = ["#", individual.id].join("/");
      if (hash !== location.hash) riot.route(hash, false);
    }

    var specs = $.extend.apply ({}, [].concat(
        individual["rdf:type"].map( function (_class) {
          return _class.specsByProps;
        })
      )
    );

    var toRender = [];

    if (template) {
      if (template instanceof veda.IndividualModel) template = $( template["v-ui:template"][0].toString() );
      if (template instanceof String) template = $( template.toString() );
      if (typeof template === "string") {
        if (template === "generic") {
          var _class = individual.hasValue("rdf:type") ? individual["rdf:type"][0] : undefined ;
          template = genericTemplate(individual, _class);
        } else if (template === "json") {
          var pre = $("<pre>"),
            json = individual.properties,
            ordered = {};
          Object.keys(json).sort().forEach(function(key) {
            ordered[key] = json[key];
          });
          json = JSON.stringify(ordered, null, 2);
          pre.text(json);
          container.html(pre);
          container.show("fade", 250);
          return;
        } else if (template === "ttl") {
          var list = new veda.IndividualListModel(individual);
          veda.Util.toTTL(list, function (error, result) {
            var ttl = $("<div class='container-fluid'></div>").append( $("<pre></pre>").text(result) );
            container.html(ttl);
            container.show("fade", 250);
          });
          return;
        } else {
          template = $( template );
        }
      }
      toRender = [ template ];

    } else {
      toRender = individual["rdf:type"].map( function (_class) {
        if (_class.template && _class.template["v-ui:template"]) {
          // Get template from class
          template = $( _class.template["v-ui:template"][0].toString() );
        } else {
          // Construct generic template
          template = genericTemplate(individual, _class);
        }
        return template;
      });
    }

    toRender.map( function (template) {
      var pre_render_src,
          pre_render,
          post_render_src,
          post_render;

      template = template.filter(function () { return this.nodeType === 1 });

      if (template.first().is("script")) {
        pre_render_src = template.first().text();
        pre_render = new Function("veda", "individual", "container", "template", "mode", pre_render_src);
      }
      if (template.last().is("script")) {
        post_render_src = template.last().text();
        post_render = new Function("veda", "individual", "container", "template", "mode", post_render_src);
      }
      template = template.filter("*:not(script)");

      if (pre_render) {
        pre_render(veda, individual, container, template, mode);
      }

      template = renderTemplate (individual, container, template, specs, mode);
      container.append(template);
      individual.trigger("individual:templateReady", template);

      // Timeout to wait all related individuals to render
      setTimeout(function () {
        template.trigger(mode);
        if (post_render) {
          post_render(veda, individual, container, template, mode);
        }
      }, 0);
    });

    if (container.prop("id") === "main") { container.show("fade", 250); }
  });

  function renderTemplate (individual, container, template, specs, mode) {

    template.attr({
      "resource": individual.id,
      "typeof": individual["rdf:type"].map(function (item) { return item.id; }).join(" ")
    });

    // Unwrapped templates support
    var wrapper = $("<div>").append(template);

    var view = $(".view", wrapper);
    var edit = $(".edit", wrapper);
    var search = $(".search", wrapper);
    var _view = $(".-view", wrapper);
    var _edit = $(".-edit", wrapper);
    var _search = $(".-search", wrapper);
    function showHideHandler (e) {
      switch (e.type) {
        case "view": view.show(); _view.hide(); break;
        case "edit": edit.show(); _edit.hide(); break;
        case "search": search.show(); _search.hide(); break;
      }
      e.stopPropagation();
    }
    template.on("view edit search", showHideHandler);

    // Embedded templates list
    var embedded = [];

    // Trigger same events for embedded templates
    function syncEmbedded (e, parent) {
      embedded.map(function (item) {
        item.trigger(e.type, individual.id);
      });
      e.stopPropagation();
    }
    template.on("view edit search save cancel delete recover draft", syncEmbedded);

    // Define handlers
    function saveHandler (e, parent) {
      if (parent !== individual.id) {
        individual.save(parent);
      }
      template.trigger("view");
      e.stopPropagation();
    }
    template.on("save", saveHandler);

    function draftHandler (e, parent) {
      if (parent !== individual.id) {
        individual.draft(parent);
      }
      template.trigger("view");
      e.stopPropagation();
    }
    template.on("draft", draftHandler);

    function showRightsHandler (e) {
      individual.trigger("showRights");
      e.stopPropagation();
    }
    template.on("showRights", showRightsHandler);

    function cancelHandler (e, parent) {
      if (parent !== individual.id) {
        individual.reset();
      }
      template.trigger("view");
      e.stopPropagation();
    }
    template.on("cancel", cancelHandler);

    // Deleted alert
    var deletedAlert = $( deletedAlertTmpl );
    var recoverBtn = $("button", deletedAlert);
    if (individual.hasValue("v-s:deleted") && individual["v-s:deleted"][0] == true) {
      afterDeleteHandler();
    }
    function afterRecoverHandler() {
      deletedAlert.remove();
      template.removeClass("deleted");
    }
    function afterDeleteHandler() {
      template.addClass("deleted");
      if ( container.prop("id") === "main" ) {
        template.prepend(deletedAlert);
        recoverBtn.click(function () {
          template.trigger("recover");
        });
      }
    }
    individual.on("individual:afterRecover", afterRecoverHandler);
    individual.on("individual:afterDelete", afterDeleteHandler);
    template.one("remove", function () {
      individual.off("individual:afterRecover", afterRecoverHandler);
      individual.off("individual:afterDelete", afterDeleteHandler);
    });

    function deleteHandler (e, parent) {
      if (parent !== individual.id) {
        individual.delete(parent);
      }
      e.stopPropagation();
    }
    template.on("delete", deleteHandler);

    function recoverHandler (e, parent) {
      if (parent !== individual.id) {
        individual.recover(parent);
      }
      e.stopPropagation();
    }
    template.on("recover", recoverHandler);

    // Actions
    var $edit = $("#edit.action", wrapper),
      $save = $("#save.action", wrapper),
      $draft = $("#draft.action", wrapper),
      $showRights = $("#rightsOrigin.action", wrapper),
      $cancel = $("#cancel.action", wrapper),
      $delete = $("#delete.action", wrapper);

    // Check rights to manage buttons
    // Update
    if ($edit.length   && !(individual.rights && individual.rights.hasValue("v-s:canUpdate") && individual.rights["v-s:canUpdate"][0] == true) ) $edit.remove();
    if ($save.length   && !(individual.rights && individual.rights.hasValue("v-s:canUpdate") && individual.rights["v-s:canUpdate"][0] == true) ) $save.remove();
    if ($draft.length  && !(individual.rights && individual.rights.hasValue("v-s:canUpdate") && individual.rights["v-s:canUpdate"][0] == true) ) $draft.remove();
    if ($cancel.length && !(individual.rights && individual.rights.hasValue("v-s:canUpdate") && individual.rights["v-s:canUpdate"][0] == true) ) $cancel.remove();
    // Delete
    if ($delete.length && ( !(individual.rights && individual.rights.hasValue("v-s:canDelete") && individual.rights["v-s:canDelete"][0] == true) || individual.isNew() ) ) $delete.remove();

    // Buttons handlers
    // Edit
    $edit.on("click", function (e) {
      e.preventDefault();
      template.trigger("edit");
    });

    // Save
    $save.on("click", function (e) {
      e.preventDefault();
      template.trigger("save");
    });

    // Draft
    $draft.on("click", function (e) {
      e.preventDefault();
      template.trigger("draft");
    });

    // Show rights
    $showRights.on("click", function (e) {
      e.preventDefault();
      template.trigger("showRights");
    });

    //  Cancel
    $cancel.on("click", function (e) {
      e.preventDefault();
      template.trigger("cancel");
    });

    //  Delete
    $delete.on("click", function (e) {
      e.preventDefault();
      if ( confirm("Вы действительно хотите удалить документ?") ) { template.trigger("delete"); }
    });
    if ( individual.hasValue("v-s:deleted", true) ) { $delete.hide(); }

    // Standart buttons labels change for drafts
    var Edit = (new veda.IndividualModel("v-s:Edit"))["rdfs:label"].join(" ");
    var ContinueEdit = (new veda.IndividualModel("v-s:ContinueEdit"))["rdfs:label"].join(" ");
    var DeleteDraft = (new veda.IndividualModel("v-s:DeleteDraft"))["rdfs:label"].join(" ");
    var Cancel = (new veda.IndividualModel("v-s:Cancel"))["rdfs:label"].join(" ");

    individual.on("individual:propertyModified", isDraftHandler);
    template.on("remove", function () {
      individual.off("individual:propertyModified", isDraftHandler);
    });

    var Draft = (new veda.IndividualModel("v-s:Draft"))["rdfs:label"].join(" ");
    var draftLabel = $("<div class='label label-danger label-draft'></div>").text(Draft);
    template.one("remove", function () {
      draftLabel.remove();
    });
    function isDraftHandler(property_uri) {
      if (property_uri === "v-s:isDraft") {
        // If individual is draft
        if ( individual.hasValue("v-s:isDraft", true) && !template.parent().closest("[resource='" + individual.id + "']").length ) {
          if (template.css("display") === "table-row" || template.prop("tagName") === "TR") {
            var cell = template.children().last();
            cell.css("position", "relative").append(draftLabel);
          } else {
            template.css("position", "relative");
            // It is important to append buttons skipping script element in template!
            template.not("script").append(draftLabel);
          }
          //Rename "Edit" -> "Continue edit"
          $edit.text(ContinueEdit);
          //Rename "Cancel" -> "Delete draft"
          $cancel.text(DeleteDraft);
        } else {
          draftLabel.remove();
          //Rename "Continue edit" -> Edit"
          $edit.text(Edit);
          //Rename "Delete draft" -> "Cancel"
          $cancel.text(Cancel);
        }
      }
    }
    setTimeout( function () {
      isDraftHandler("v-s:isDraft");
    }, 100);

    // Apply mode to template to show/hide elements in different modes
    function modeHandler (e) {
      mode = e.type;
      mode === "view" ? template.addClass("mode-view").removeClass("mode-edit mode-search") :
      mode === "edit" && (individual.rights && individual.rights.hasValue("v-s:canUpdate") && individual.rights["v-s:canUpdate"][0] == true) ? template.addClass("mode-edit").removeClass("mode-view mode-search") :
      mode === "search" ? template.addClass("mode-search").removeClass("mode-view mode-edit") :
      true;
      template.attr("data-mode", mode);
      e.stopPropagation();
    }
    template.on("view edit search", modeHandler);

    // Additional actions buttons
    var $send = $("#send.action", wrapper);
    var $sendButtons = $(".sendbutton", wrapper);
    var $createReport = $("#createReport.action", wrapper);
    var $createReportButtons = $(".create-report-button", wrapper);
    var $showRights = $("#rightsOrigin.action", wrapper);
    var $journal = $("#journal.action", wrapper);

    function validHandler(e) {
      $save.removeAttr("disabled");
      $send.removeAttr("disabled");
      $sendButtons.removeAttr("disabled");
      $createReport.removeAttr("disabled");
      $createReportButtons.removeAttr("disabled");
      e.stopPropagation();
    }
    function inValidHandler(e) {
      $save.attr("disabled", "disabled");
      $send.attr("disabled", "disabled");
      $sendButtons.attr("disabled", "disabled");
      $createReport.attr("disabled", "disabled");
      $createReportButtons.attr("disabled", "disabled");
      e.stopPropagation();
    }
    template.on("valid", validHandler);
    template.on("invalid", inValidHandler);

    $send.on("click", function () {veda.Util.send(individual, template);});
    $createReport.on("click", function () {veda.Util.createReport(individual);});
    $showRights.on("click", function () {veda.Util.showRights(individual);});
    $journal.on("click", function() {
      var journal_uri = individual.id + "j",
          journal = new veda.IndividualModel(journal_uri);
      if (journal.hasValue("rdf:type") && journal["rdf:type"][0].id !== "rdfs:Resource") {
        riot.route("#/" + journal_uri);
      } else {
        alert("Журнал отсутсвует / Journal empty");
      }
    });

    // standard tasks
    $('ul#standard-tasks', template).each(function() {
      var stask = $(this);
      stask.append($('<li/>', {
        style:'cursor:pointer',
        click: function() {veda.Util.send(individual, template, 'v-wf:questionRouteStartForm', true)},
        html: '<a>'+(new veda.IndividualModel('v-s:SendQuestion')['rdfs:label'][0])+'</a>'
      }));
      stask.append($('<li/>', {
        style:'cursor:pointer',
        click: function() {veda.Util.send(individual, template, 'v-wf:instructionRouteStartForm', true)},
        html: '<a>'+(new veda.IndividualModel('v-s:SendInstruction')['rdfs:label'][0])+'</a>'
      }));
    });

    // Process RDFa compliant template

    // Special (not RDFa)
    $("[href*='@']:not([rel] *):not([about] *)", wrapper).map( function () {
      var self = $(this);
      var str = self.attr("href");
      self.attr("href", str.replace("@", individual.id));
    });

    $("[src*='@']:not([rel] *):not([about] *)", wrapper).map( function () {
      var self = $(this);
      var str = self.attr("src");
      self.attr("src", str.replace("@", individual.id));
    });

    // Property value
    var props_ctrls = {};
    $("[property]:not(veda-control):not([rel] *):not([about]):not([about] *)", wrapper).map( function () {
      var propertyContainer = $(this),
        property_uri = propertyContainer.attr("property"),
        spec = specs[property_uri];
      if (property_uri === "@") {
        propertyContainer.text(individual.id);
        return;
      }
      propertyModifiedHandler(property_uri);
      // Re-render all property values at propertyModified event from model
      function propertyModifiedHandler(doc_property_uri) {
        if (doc_property_uri === property_uri) {
          renderPropertyValues(individual, property_uri, propertyContainer, props_ctrls, template, mode);
        }
      }
      individual.on("individual:propertyModified", propertyModifiedHandler);
      template.one("remove", function () {
        individual.off("individual:propertyModified", propertyModifiedHandler);
      });
    });

    // Related resources & about resources
    $("[rel]:not(veda-control):not([rel] *):not([about] *)", wrapper).map( function () {
      var relContainer = $(this),
        about = relContainer.attr("about"),
        rel_uri = relContainer.attr("rel"),
        isEmbedded = relContainer.attr("data-embedded") === "true",
        spec = specs[rel_uri],
        rel_inline_template = relContainer.children(),
        rel_template_uri = relContainer.attr("data-template"),
        relTemplate,
        isAbout;

      var sortableOptions = {
        delay: 150,
        placeholder: "sortable-placeholder",
        forcePlaceholderSize: true,
        handle: ".button-drag",
        cancel: "",
        update: function () {
          var uris = $(this).sortable("toArray", {attribute: "resource"});
          individual[rel_uri] = uris.map(function (uri) {
            return new veda.IndividualModel(uri);
          });
        }
      };
      relContainer.sortable(sortableOptions);

      if (about) {
        isAbout = true;
        about = (about === "@" ? individual : new veda.IndividualModel(about));
        relContainer.attr("about", about.id);
      } else {
        isAbout = false;
        about = individual;
      }

      if ( rel_template_uri ) {
        var templateIndividual = new veda.IndividualModel( rel_template_uri );
        relTemplate = $( templateIndividual["v-ui:template"][0].toString() );
      }
      if ( rel_inline_template.length ) {
        relTemplate = rel_inline_template.remove();
      }
      rel_inline_template = null;

      template.on("view edit search", function (e) {
        if (e.type === "view") {
          relContainer.sortable("disable");
        }
        else if (e.type === "edit") {
          relContainer.sortable("enable");
          var property = new veda.IndividualModel(rel_uri);
          if ( isEmbedded
             && spec
             && spec["v-ui:minCardinality"][0] >= 1
             && !individual.hasValue(rel_uri)
             && !(property.hasValue("rdfs:range") && property["rdfs:range"][0].id === "v-s:File")
          ) {
            var valueType = spec && spec.hasValue("v-ui:rangeRestriction") ?
              spec["v-ui:rangeRestriction"] : property.hasValue("rdfs:range") ?
              property["rdfs:range"]        : [];
            var emptyValue = new veda.IndividualModel();
            if ( valueType.length ) {
              emptyValue["rdf:type"] = valueType;
            }
            individual[rel_uri] = [emptyValue];
          }
        }
        else if (e.type === "search") {
          relContainer.sortable("disable");
        }
        e.stopPropagation();
      });

      var values = about[rel_uri], rendered = {}, counter = 0;

      relContainer.empty();

      propertyModifiedHandler(rel_uri, values);
      about.on("individual:propertyModified", propertyModifiedHandler);
      template.one("remove", function () {
        about.off("individual:propertyModified", propertyModifiedHandler);
      });

      if (isEmbedded) {
        embeddedHandler(rel_uri, values);
        about.on("individual:propertyModified", embeddedHandler);
        template.one("remove", function () {
          about.off("individual:propertyModified", embeddedHandler);
        });
      }

      // Re-render link property if its' values were changed
      function propertyModifiedHandler (doc_rel_uri, values) {
        if (doc_rel_uri === rel_uri) {
          ++counter;
          if (values.length) {
            values.map(function (value) {
              if (value.id in rendered) {
                rendered[value.id].cnt = counter;
                return;
              }
              setTimeout (function () {
                var renderedTmpl = renderRelationValue (about, rel_uri, value, relContainer, relTemplate, isEmbedded, embedded, isAbout, template, mode);
                rendered[value.id] = {tmpl: renderedTmpl, cnt: counter};
              }, 0);
            });
          } else {
            relContainer.empty();
          }
          // Remove rendered templates for removed values
          for (var i in rendered) {
            if (rendered[i].cnt === counter) continue;
            rendered[i].tmpl.remove();
            delete rendered[i];
          }
        }
      }

      function embeddedHandler(doc_rel_uri, values) {
        if (doc_rel_uri === rel_uri && mode === "edit") {
          values.map(function (value) {
            if (
              value.id !== about.id // prevent self parent
              && rel_uri !== "v-s:parent" // prevent circular parent
              && !value.hasValue("v-s:parent") // do not change parent
            ) {
              value["v-s:parent"] = [about];
            }
          });
        }
      }

    });

    // About resource
    $("[about]:not([rel]):not([property])", wrapper).map( function () {
      var aboutContainer = $(this),
        about_template_uri = aboutContainer.attr("data-template"),
        about_inline_template = aboutContainer.children(),
        isEmbedded = aboutContainer.attr("data-embedded") === "true",
        about, aboutTemplate;
      if ( about_template_uri ) {
        var templateIndividual = new veda.IndividualModel( about_template_uri );
        aboutTemplate = $( templateIndividual["v-ui:template"][0].toString() );
      }
      if ( about_inline_template.length ) {
        aboutTemplate = about_inline_template.remove();
      }
      if (aboutContainer.attr("about") === "@") {
        about = individual;
        aboutContainer.attr("about", about.id);
      } else {
        about = new veda.IndividualModel(aboutContainer.attr("about"));
      }
      aboutContainer.empty();
      about.present(aboutContainer, aboutTemplate);
      if (isEmbedded) {
        aboutTemplate = $("[resource='" + about.id + "']", aboutContainer);
        aboutTemplate.data("isEmbedded", true);
        embedded.push(aboutTemplate);
      }
    });

    // About resource property
    $("[about][property]:not([rel] *):not([about] *)", wrapper).map( function () {
      var propertyContainer = $(this),
        property_uri = propertyContainer.attr("property"),
        about;
      if (propertyContainer.attr("about") === "@") {
        about = individual;
        propertyContainer.attr("about", about.id);
      } else {
        about = new veda.IndividualModel(propertyContainer.attr("about"));
      }
      propertyModifiedHandler(property_uri);
      function propertyModifiedHandler(doc_property_uri) {
        if (doc_property_uri === property_uri) {
          if (property_uri === "@") {
            propertyContainer.text( about.id );
          } else if (about[property_uri] !== undefined) {
            var formatted = about[property_uri].map(veda.Util.formatValue).join(" ");
            propertyContainer.text( formatted );
          }
        }
      }
      about.on("individual:propertyModified", propertyModifiedHandler);
      template.one("remove", function () {
        about.off("individual:propertyModified", propertyModifiedHandler);
      });
      veda.on("language:changed", langWatch);
      template.one("remove", function () {
        veda.off("language:changed", langWatch);
      });
      function langWatch () {
        propertyModifiedHandler(property_uri);
      }
      var updateService = new veda.UpdateService();
      updateService.subscribe(about.id);
      template.one("remove", function () {
        updateService.unsubscribe(about.id);
      });
    });

    // Validation with support of embedded templates (arbitrary depth)

    // Initial validation state
    var validation = {};
    template.data({
      "valid": true,
      "validation": validation
    });

    function validateTemplate (e) {
      if ( !Object.keys(validation).length ) {
        e.stopPropagation();
        return;
      }
      if (mode === "edit") {
        var isValid = Object.keys(validation).reduce( function (acc, property_uri) {
          if ( !validation[property_uri].isCustom ) {
            validation[property_uri] = validate(individual, property_uri, specs[property_uri]);
          }
          return acc && validation[property_uri].state;
        }, true);
        isValid = isValid && embedded.reduce(function (acc, template) {
          return acc && template.data("valid");
        }, true);
        //console.log("validate handler", individual.id, validation);
        //console.log("validate handler", ++c1);
        template.data("valid", isValid);
        template.trigger(isValid ? "valid" : "invalid", validation);
      }
      // "validate" event should bubble up to be handled by parent template only if current template is embedded
      if ( !template.data("isEmbedded") ) {
        e.stopPropagation();
      }
    }
    template.on("validate", validateTemplate);
    //var c1 = 0;

    function triggerValidation() {
      if (mode === "edit") {
        template.trigger("validate");
      }
    };
    individual.on("individual:propertyModified", triggerValidation);
    template.one("remove", function () {
      individual.off("individual:propertyModified", triggerValidation);
    });
    template.on("edit", triggerValidation);

    // Handle validation events from template
    template.on("valid invalid", function (e, validationResult) {
      e.stopPropagation();
      if (mode === "edit") {
        if (validationResult !== validation) {
          // Remove previous custom validation results
          Object.keys(validation).map(function (property_uri) {
            if (validation[property_uri].isCustom) {
              delete validation[property_uri];
            }
          });
          // Merge custom validation results with standard results
          for (var property_uri in validationResult) {
            validation[property_uri] = validationResult[property_uri];
            validation[property_uri].isCustom = true;
          }
          var isValid = template.data("valid") && (e.type === "valid");
          template.data("valid", isValid);
          e.type = isValid ? "valid" : "invalid";
        }
        // trigger validation in parent template if this template is embedded
        if ( template.data("isEmbedded") && template.parent().length) {
          template.parent().trigger("validate");
        }
        //console.log("valid-invalid handler", individual.id, validation);
        //console.log("valid-invalid handler", ++c2);
      }
    });

    //var c2 = 0;

    // Property control
    $("veda-control[property]:not([rel] *):not([about] *)", wrapper).map( function () {

      var control = $(this),
        property_uri = control.attr("property"),
        property = new veda.IndividualModel(property_uri),
        type = control.attr("data-type") || property["rdfs:range"][0].id,
        spec = specs[property_uri],
        controlType = control.attr("data-type") ? $.fn["veda_" + control.attr("data-type")] : $.fn.veda_generic;

      //control.removeAttr("property");

      // Initial validation state
      validation[property_uri] = {state: true, cause: []};

      function validationHandler(e) {
        if ( e.type === "valid" || !validation[property_uri] || validation[property_uri].state === true ) {
          control.removeClass("has-error");
          control.popover("destroy");
        } else {
          control.addClass("has-error");
          control.popover({
            content: function () {
              return validation[property_uri].cause.map(function (cause_uri) {
                return (new veda.IndividualModel(cause_uri))["rdfs:comment"].join(", ");
              }).join("\n");
            },
            container: control,
            trigger: "hover focus",
            placement: "top",
            animation: false
          });
          if ( $("input", control).is(":focus") ) {
            control.popover("show");
          }
        }
        e.stopPropagation();
      }
      template.on("valid invalid", validationHandler);

      template.on("view edit search", function (e) {
        e.stopPropagation();
        control.trigger(e.type);
      });

      var opts = {
        individual: individual,
        property_uri: property_uri,
        spec: spec,
        mode: mode
      };

      controlType.call(control, opts);

      props_ctrls[property_uri] ? props_ctrls[property_uri].push(control) : props_ctrls[property_uri] = [ control ];

      function assignDefaultValue (e) {
        if ( spec && !individual.hasValue(property_uri) ) {
          var defaultValue;
          switch (property["rdfs:range"][0].id) {
            case "xsd:boolean":
              defaultValue = spec && spec.hasValue("v-ui:defaultBooleanValue") ? spec["v-ui:defaultBooleanValue"][0] : undefined;
              break;
            case "xsd:integer":
            case "xsd:nonNegativeInteger":
              defaultValue = spec && spec.hasValue("v-ui:defaultIntegerValue") ? spec["v-ui:defaultIntegerValue"][0] : undefined;
              break;
            case "xsd:decimal":
              defaultValue = spec && spec.hasValue("v-ui:defaultDecimalValue") ? spec["v-ui:defaultDecimalValue"][0] : undefined;
              break;
            case "xsd:dateTime":
              defaultValue = spec && spec.hasValue("v-ui:defaultDatetimeValue") ? spec["v-ui:defaultDatetimeValue"][0] : undefined;
              break;
            default:
              defaultValue = spec && spec.hasValue("v-ui:defaultStringValue") ? spec["v-ui:defaultStringValue"][0] : undefined;
              break;
          }
          if (defaultValue !== undefined) individual[property_uri] = [ defaultValue ];
        }
        e.stopPropagation();
      }
      template.on("edit", assignDefaultValue);

    });

    // Relation control
    $("veda-control[rel]:not([rel] *):not([about] *)", wrapper).map( function () {

      var control = $(this),
        rel_uri = control.attr("rel"),
        spec = specs[rel_uri],
        rel = new veda.IndividualModel(rel_uri),
        controlType = control.attr("data-type") ? $.fn["veda_" + control.attr("data-type")] : $.fn.veda_link;

      //control.removeAttr("rel");

      // Initial validation state
      validation[rel_uri] = {state: true, cause: []};

      function validationHandler(e) {
        if ( e.type === "valid" || !validation[rel_uri] || validation[rel_uri].state === true) {
          control.removeClass("has-error");
          control.popover("destroy");
        } else {
          control.addClass("has-error");
          control.popover({
            content: function () {
              return validation[rel_uri].cause.map(function (cause_uri) {
                return (new veda.IndividualModel(cause_uri))["rdfs:comment"].join(", ");
              }).join("\n");
            },
            container: control,
            trigger: "hover focus",
            placement: "top",
            animation: false
          });
          if ( $("input", control).is(":focus") ) {
            control.popover("show");
          }
        }
        e.stopPropagation();
      }
      template.on("valid invalid", validationHandler);

      template.on("view edit search", function (e) {
        e.stopPropagation();
        control.trigger(e.type);
      });

      var opts = {
        individual: individual,
        rel_uri: rel_uri,
        spec: spec,
        mode: mode
      };

      controlType.call(control, opts);

      function modeHandler(e) {
        control.trigger(e.type);
      }
      template.on("view edit search", modeHandler);

      function assignDefaultObjectValue (e) {
        if ( spec && spec.hasValue("v-ui:defaultObjectValue") && !individual.hasValue(rel_uri) ) {
          individual[rel_uri] = [ spec["v-ui:defaultObjectValue"][0] ];
        }
        e.stopPropagation();
      }
      template.on("edit", assignDefaultObjectValue);

      // tooltip from spec
      if (spec && spec.hasValue("v-ui:tooltip")) {
        control.tooltip({
          title: spec["v-ui:tooltip"].join(", "),
          placement: "top",
          container: control,
          trigger: "focus",
          animation: false
        });
      }

    });

    return template;
  }

  function renderPropertyValues(individual, property_uri, propertyContainer, props_ctrls, template, mode) {
    propertyContainer.empty();
    individual[property_uri].map( function (value, i) {
      var valueHolder = $("<span class='value-holder'/>");
      propertyContainer.append(valueHolder.text( veda.Util.formatValue(value) ));
      var wrapper = $("<div id='prop-actions' class='btn-group btn-group-xs' role='group'></div>");
      var btnEdit = $("<button class='btn btn-default'><span class='glyphicon glyphicon-pencil'></span></button>");
      var btnRemove = $("<button class='btn btn-default'><span class='glyphicon glyphicon-remove'></span></button>");
      wrapper.append(btnEdit, btnRemove);

      template.on("view edit search", function (e) {
        if (e.type === "view") wrapper.hide();
        else wrapper.show();
        e.stopPropagation();
      });
      if (mode === "view") { wrapper.hide(); }

      btnRemove.click(function () {
        individual[property_uri] = individual[property_uri].filter(function (_, j) {return j !== i; });
      }).mouseenter(function () {
        valueHolder.addClass("red-outline");
      }).mouseleave(function () {
        valueHolder.removeClass("red-outline");
      });
      btnEdit.click(function () {
        var val;
        individual[property_uri] = individual[property_uri].filter(function (_, j) {
          var test = j !== i;
          if (!test) val = individual[property_uri][j];
          return test;
        });
        if ( props_ctrls[property_uri] ) {
          props_ctrls[property_uri].map(function (item, i) {
            item.val(val);
            if (i === 0) item.trigger("veda_focus", [val]);
          });
        }
      }).mouseenter(function () {
        valueHolder.addClass("blue-outline");
      }).mouseleave(function () {
        valueHolder.removeClass("blue-outline");
      });
      //valueHolder.after( wrapper );
      valueHolder.append( wrapper );
    });
  }

  function renderRelationValue(individual, rel_uri, value, relContainer, relTemplate, isEmbedded, embedded, isAbout, template, mode) {
    var valTemplate;
    if (isEmbedded) {
      if (relTemplate) {
        valTemplate = relTemplate.clone();
        value.present(relContainer, valTemplate, mode);
      } else {
        value.present(relContainer, undefined, mode);
      }
      valTemplate = $("[resource='" + value.id + "']", relContainer).first();
      valTemplate.data("isEmbedded", true);
      embedded.push(valTemplate);
      valTemplate.on("remove", function () {
        if (embedded.length) {
          var index = embedded.indexOf(valTemplate);
          if ( index >= 0 ) embedded.splice(index, 1);
        }
      });
    } else {
      if (relTemplate) {
        valTemplate = relTemplate.clone();
        value.present(relContainer, valTemplate);
      } else {
        value.present(relContainer);
      }
      valTemplate = $("[resource='" + value.id + "']", relContainer).first();
    }
    if (!isAbout) {
      var wrapper = $("<div id='rel-actions' class='btn-group btn-group-xs -view edit search' role='group'></div>");
      var btnDrag = $("<button class='btn btn-default button-drag'><span class='glyphicon glyphicon-move'></span></button>");
      var btnRemove = $("<button class='btn btn-default button-delete'><span class='glyphicon glyphicon-remove'></span></button>");
      wrapper.append(btnDrag, btnRemove);
      template.on("view edit search", function (e) {
        if (e.type === "view") wrapper.hide();
        else wrapper.show();
        e.stopPropagation();
      });
      if (mode === "view") { wrapper.hide(); }

      if (valTemplate.attr("deleteButton") == "hide") {
        btnRemove.hide();
      }
      btnRemove.click(function () {
        individual[rel_uri] = individual[rel_uri].filter(function (item) { return item.id !== value.id; });
      }).mouseenter(function () {
        valTemplate.addClass("red-outline");
      }).mouseleave(function () {
        valTemplate.removeClass("red-outline");
      });

      //Sortable scroll bugfix
      btnDrag.mouseenter(function () {
        valTemplate.addClass("gray-outline");
      }).mouseleave(function () {
        valTemplate.removeClass("gray-outline");
      }).mousedown(function () {
        relContainer.addClass("sortable-overflow");
      }).mouseup(function () {
        relContainer.removeClass("sortable-overflow");
      });

      if (valTemplate.css("display") !== "inline") {
        wrapper.addClass("block");
      }
      if (valTemplate.css("display") === "table-row" || valTemplate.prop("tagName") === "TR") {
        var cell = valTemplate.children().last();
        cell.css("position", "relative").append(wrapper);
      } else {
        valTemplate.css("position", "relative");
        valTemplate.append(wrapper);
      }
    }
    return valTemplate;
  }

  // Property validation according to specification
  function validate(individual, property_uri, spec) {
    var result = {
      state: true,
      cause: []
    };
    if (!spec) { return result; }
    var values = individual[property_uri];
    // cardinality check
    if (spec.hasValue("v-ui:minCardinality")) {
      var minCardinalityState = (values.length >= spec["v-ui:minCardinality"][0] &&
        // filter empty values
        values.length === values.filter(function(item) {
          return (
            typeof item === "boolean" ? true :
            typeof item === "number" ? true : !!item
          ) ;
        }).length);
      result.state = result.state && minCardinalityState;
      if (!minCardinalityState) {
        result.cause.push("v-ui:minCardinality");
      }
    }
    if (spec.hasValue("v-ui:maxCardinality")) {
      var maxCardinalityState = (
        values.length <= spec["v-ui:maxCardinality"][0] &&
        // filter empty values
        values.length === values.filter(function(item) {
          return (
            typeof item === "boolean" ? true :
            typeof item === "number" ? true : !!item
          ) ;
        }).length
      );
      result.state = result.state && maxCardinalityState;
      if (!maxCardinalityState) {
        result.cause.push("v-ui:maxCardinality");
      }
    }
    // check each value
    result = result && values.reduce(function (result, value) {
      // regexp check
      if (spec.hasValue("v-ui:regexp")) {
        var regexp = new RegExp(spec["v-ui:regexp"][0]);
        var regexpState = regexp.test(value.toString());
        result.state = result.state && regexpState;
        if (!regexpState) {
          result.cause.push("v-ui:regexp");
        }
      }
      // range check
      switch (spec["rdf:type"][0].id) {
        case "v-ui:IntegerPropertySpecification" :
          if (spec.hasValue("v-ui:minIntegerValue")) {
            var minIntegerValueState = (value >= spec["v-ui:minIntegerValue"][0]);
            result.state = result.state && minIntegerValueState;
            if (!minIntegerValueState) {
              result.cause.push("v-ui:minIntegerValue");
            }
          }
          if (spec.hasValue("v-ui:maxIntegerValue")) {
            var maxIntegerValueState = (value <= spec["v-ui:maxIntegerValue"][0]);
            result.state = result.state && maxIntegerValueState;
            if (!maxIntegerValueState) {
              result.cause.push("v-ui:maxIntegerValue");
            }
          }
          break;
        case "v-ui:DecimalPropertySpecification" :
          if (spec.hasValue("v-ui:minDecimalValue")) {
            var minDecimalValueState = (value >= spec["v-ui:minDecimalValue"][0]);
            result.state = result.state && minDecimalValueState;
            if (!minDecimalValueState) {
              result.cause.push("v-ui:minDecimalValue");
            }
          }
          if (spec.hasValue("v-ui:maxDecimalValue")) {
            var maxDecimalValueState = (value <= spec["v-ui:maxDecimalValue"][0]);
            result.state = result.state && maxDecimalValueState;
            if (!maxDecimalValueState) {
              result.cause.push("v-ui:maxDecimalValue");
            }
          }
          break;
        case "v-ui:DatetimePropertySpecification" :
          if (spec.hasValue("v-ui:minDatetimeValue")) {
            var minDatetimeValueState = (value >= spec["v-ui:minDatetimeValue"][0]);
            result.state = result.state && minDatetimeValueState;
            if (!minDatetimeValueState) {
              result.cause.push("v-ui:minDatetimeValue");
            }
          }
          if (spec.hasValue("v-ui:maxDatetimeValue")) {
            var maxDatetimeValueState = (value <= spec["v-ui:maxDatetimeValue"][0]);
            result.state = result.state && maxDatetimeValueState;
            if (!maxDatetimeValueState) {
              result.cause.push("v-ui:maxDatetimeValue");
            }
          }
          break;
        case "v-ui:StringPropertySpecification" :
          if (spec.hasValue("v-ui:minLength")) {
            var minLengthState = (value.length >= spec["v-ui:minLength"][0]);
            result.state = result.state && minLengthState;
            if (!minLengthState) {
              result.cause.push("v-ui:minLength");
            }
          }
          if (spec.hasValue("v-ui:maxLength")) {
            var maxLengthState = (value.length <= spec["v-ui:maxLength"][0]);
            result.state = result.state && maxLengthState;
            if (!maxLengthState) {
              result.cause.push("v-ui:maxLength");
            }
          }
          break;
        case "v-ui:PropertySpecification" :
        case "v-ui:BooleanPropertySpecification" :
        case "v-ui:ObjectPropertySpecification" :
          break;
      }
      return result;
    }, result);
    return result;
  }

  function genericTemplate (individual, _class) {
    // Construct generic template
    var propTmpl = $("#generic-property-template").html();
    var template = $("<div/>").append( $("#generic-class-template").html() );
    var properties;

    if (_class) {
      properties = _class.domainProperties;
      $(".className", template).append (
        $("<span/>", {"about": _class.id, "property": "rdfs:label"})
      );
    } else {
      properties = individual.properties;
    }
    $(".properties", template).append (
      Object.getOwnPropertyNames(properties).map( function (property_uri, index, array) {

        if (property_uri === "@" || property_uri === "rdfs:label" || property_uri === "rdf:type" || property_uri === "v-s:deleted") { return; }

        var property = new veda.IndividualModel(property_uri);

        var result = $("<div/>").append( propTmpl );
        $(".name", result).append (
          $("<strong/>", {"about": property_uri, "property": "rdfs:label"}).addClass("text-muted")
        );

        var range = property["rdfs:range"] ? property["rdfs:range"][0].id : "rdfs:Literal";

        switch ( range ) {
          case "rdfs:Literal":
          case "xsd:string":
            if (property_uri === "v-s:script" || property_uri === "v-ui:template") {
              $(".value", result).append (
                "<veda-control property='" + property_uri + "' data-type='source'></veda-control>"
              );
            } else {
              $(".value", result).append (
                "<div property='" + property_uri + "' class='view -edit -search'/>" +
                "<veda-control property='" + property_uri + "' data-type='multilingualText' class='-view edit search'></veda-control>"
              );
            }
            break;
          case "xsd:integer":
          case "xsd:nonNegativeInteger":
            $(".value", result).append (
              "<div property='" + property_uri + "' />" +
              "<veda-control property='" + property_uri + "' data-type='integer' class='-view edit search'></veda-control>"
            );
            break;
          case "xsd:decimal":
            $(".value", result).append (
              "<div property='" + property_uri + "' />" +
              "<veda-control property='" + property_uri + "' data-type='decimal' class='-view edit search'></veda-control>"
            );
            break;
          case "xsd:dateTime":
            $(".value", result).append (
              "<div property='" + property_uri + "' />" +
              "<veda-control property='" + property_uri + "' data-type='dateTime' class='-view edit search'></veda-control>"
            );
            break;
          case "xsd:boolean":
            $(".name", result).empty();
            $(".value", result).append (
              "<div class='checkbox'>" +
                "<label>" +
                  "<veda-control property='" + property_uri + "' data-type='booleanCheckbox'></veda-control>" +
                  "<em about='" + property_uri + "' property='rdfs:label' class='text-muted'></em>" +
                "</label>" +
              "</div>"
            );
            break;
          case "rdfs:Resource":
            $(".value", result).append (
              "<div property='" + property_uri + "' />" +
              "<veda-control property='" + property_uri + "' data-type='generic' class='-view edit search'></veda-control>"
            );
            break;
          default:
            if (property_uri === "v-s:attachment") {
              $(".value", result).append (
                "<div rel='" + property_uri + "' data-template='v-ui:FileTemplateWithComment' data-embedded='true' />" +
                "<veda-control rel='" + property_uri + "' data-type='file' class='-view edit -search'></veda-control>"
              );
            } else {
              $(".value", result).append (
                "<div rel='" + property_uri + "' data-template='v-ui:ClassNameLabelLinkTemplate' />" +
                "<veda-control rel='" + property_uri + "' data-type='link' class='-view edit search fullsearch fulltext dropdown'></veda-control>"
              );
            }
            break;
        }
        if (index < array.length-1) result.append( $("<hr/>").attr("style", "margin: 10px 0px") );

        return result;
      })
    );
    return template;
  }
});

