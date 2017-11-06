// Individual Presenter Async

veda.Module(function (veda) { "use strict";

  //var c = 0;

  veda.IndividualPresenterAsync = function (container, template, mode) {

    var individual = this;
    
    //console.log(individual.id, "presenter count:", ++c);

    if (typeof container === "string") {
      container = $(container).empty();
    }
    mode = mode || "view";

    if (container.prop("id") === "main") { container.hide(); }

    var ontology = new veda.OntologyModel();

    var specs = $.extend.apply (
      {}, [].concat(
        individual["rdf:type"].map( function (_class) {
          return ontology.getClassSpecifications(_class.id);
        })
      )
    );

    if (template) {
      if (template instanceof veda.IndividualModelAsync) {
        template.load().then(function (template) {
          template = $( template["v-ui:template"][0].toString() );
          renderTemplate(individual, container, template, mode, specs);
        });
      } else if (template instanceof veda.IndividualModel) {
        template = $( template["v-ui:template"][0].toString() );
        renderTemplate(individual, container, template, mode, specs);
      } else if (typeof template === "string") {
        template = new veda.IndividualModelAsync(template);
        template.load().then(function (template) {
          template = $( template["v-ui:template"][0].toString() );
          renderTemplate(individual, container, template, mode, specs);
        });
      } else {
        renderTemplate(individual, container, template, mode, specs);
      }
    } else {
      individual["rdf:type"].map(function (type) {
        type.load()
          .then(function (type) {
            return type.hasValue("v-ui:hasTemplate") ? type["v-ui:hasTemplate"][0].load() : (new veda.IndividualModelAsync("v-ui:generic")).load();
          })
          .then(function (template) {
            template = $( template["v-ui:template"][0].toString() );
            renderTemplate(individual, container, template, mode, specs);
          });
      });
    }

    if (container.prop("id") === "main") { container.show("fade", 250); }
  }

  function renderTemplate(individual, container, template, mode, specs) {
    var pre_render_src,
        pre_render,
        post_render_src,
        post_render;

    template = template.filter(function () { return this.nodeType === 1 });

    if (template.first().is("script")) {
      pre_render_src = template.first().text();
      pre_render = new Function("veda", "individual", "container", "template", "mode", "specs", "\"use strict\";" + pre_render_src);
    }
    if (template.last().is("script")) {
      post_render_src = template.last().text();
      post_render = new Function("veda", "individual", "container", "template", "mode", "specs", "\"use strict\";" + post_render_src);
    }
    template = template.filter("*:not(script)");

    if (pre_render) {
      pre_render.call(individual, veda, individual, container, template, mode, specs);
    }

    template = processTemplate (individual, container, template, mode, specs);
    container.append(template);
    individual.trigger("individual:templateReady", template);

    // Timeout to wait all related individuals to render
    template.trigger(mode);
    if (post_render) {
      post_render.call(individual, veda, individual, container, template, mode, specs);
    }
  }

  function processTemplate (individual, container, template, mode, specs) {

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

    // Apply mode to template to show/hide elements in different modes
    function modeHandler (e) {
      mode = e.type;
      template.data("mode", mode);
      switch (e.type) {
        case "view": view.show(); _view.hide(); break;
        case "edit": edit.show(); _edit.hide(); break;
        case "search": search.show(); _search.hide(); break;
      }
      e.stopPropagation();
    }
    template.on("view edit search", modeHandler);

    // Embedded templates list
    var embedded = [];

    // Trigger same events for embedded templates
    function syncEmbedded (e) {
      embedded.map(function (item) {
        item.trigger(e.type, individual.id);
      });
      e.stopPropagation();
    }
    template.on("view edit search save cancel delete recover draft", syncEmbedded);

    // Define handlers

    var notify = veda.Notify ? new veda.Notify() : function () {};

    function saveHandler (e, parent) {
      e.stopPropagation();
      if (parent !== individual.id) {
        individual.save().then(
          function () {
            template.trigger("view");
            notify("success", {name: "Объект сохранен"});
          }, function (error) {
            notify("danger", {name: "Объект не сохранен"});
          }
        );
      }
    }
    template.on("save", saveHandler);

    function draftHandler (e, parent) {
      if (parent !== individual.id) {
        individual.draft();
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
      template.trigger("view");
      if (parent !== individual.id) {
        individual.reset();
        if (container.prop("id") === "main") {
          window.history.back();
        }
      }
      e.stopPropagation();
    }
    template.on("cancel", cancelHandler);

    // Deleted alert
    function deletedRecoveredHandler() {
      if ( individual.hasValue("v-s:deleted", true) ) {
        template.addClass("deleted");
        if ( container.prop("id") === "main" ) {
          $("<div class='deleted-alert alert alert-warning no-margin' role='alert'>\
               <p>Объект удален.  <button class='btn btn-default btn-sm'>Восстановить</button></p>\
             </div>")
          .prependTo(template)
          .find("button").click(function () {
            template.trigger("recover");
          });
        }
      } else {
        template.removeClass("deleted");
        if ( container.prop("id") === "main" ) {
          $(".deleted-alert", template).remove();
        }
      }
    }
    individual.on("v-s:deleted", deletedRecoveredHandler);
    template.one("remove", function () {
      individual.off("v-s:deleted", deletedRecoveredHandler);
    });
    deletedRecoveredHandler();

    function deleteHandler (e, parent) {
      if (parent !== individual.id) {
        individual.delete();
      }
      e.stopPropagation();
    }
    template.on("delete", deleteHandler);

    function recoverHandler (e, parent) {
      if (parent !== individual.id) {
        individual.recover();
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
    var canUpdate = individual.rights.hasValue("v-s:canUpdate", true);
    var canDelete = individual.rights.hasValue("v-s:canDelete", true);
    if (!canUpdate) {
      $edit.remove();
      $save.remove();
      $draft.remove();
      $cancel.remove();
    }
    if (!canDelete || individual.isNew()) {
      $delete.remove();
    }

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
      if ( confirm("Вы уверены? / Are you sure?") ) { template.trigger("delete"); }
    });

    // Standart buttons labels change for drafts
    var Edit = (new veda.IndividualModel("v-s:Edit"))["rdfs:label"].join(" ");
    var ContinueEdit = (new veda.IndividualModel("v-s:ContinueEdit"))["rdfs:label"].join(" ");
    var DeleteDraft = (new veda.IndividualModel("v-s:DeleteDraft"))["rdfs:label"].join(" ");
    var Cancel = (new veda.IndividualModel("v-s:Cancel"))["rdfs:label"].join(" ");

    var Draft = (new veda.IndividualModel("v-s:Draft"))["rdfs:comment"].join(" ");
    var draftLabel = null;
    function isDraftHandler(property_uri) {
      if (property_uri === "v-s:isDraft") {
        // If individual is draft
        if ( individual.hasValue("v-s:isDraft", true) ) {
          if ( !template.parent().closest("[resource='" + individual.id + "']").length && !draftLabel ) {
            draftLabel = $("<div class='label label-default label-draft'></div>").text(Draft);
            if (template.css("display") === "table-row" || template.prop("tagName") === "TR") {
              var cell = template.children().first();
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
          }
        } else {
          if (draftLabel) {
            draftLabel.remove();
            draftLabel = null;
          }
          //Rename "Continue edit" -> Edit"
          $edit.text(Edit);
          //Rename "Delete draft" -> "Cancel"
          $cancel.text(Cancel);
        }
      } else {
        if (mode === "edit") {
          individual.draft();
        }
      }
    }
    individual.on("propertyModified", isDraftHandler);
    template.one("remove", function () {
      individual.off("propertyModified", isDraftHandler);
      draftLabel = null;
    });

    setTimeout(function () {
      isDraftHandler("v-s:isDraft");
    }, 0);

    // Additional actions buttons
    var $send = $("#send.action", wrapper);
    var $sendButtons = $(".sendbutton", wrapper);
    var $createReport = $("#createReport.action", wrapper);
    var $createReportButtons = $(".create-report-button", wrapper);
    var $showRights = $("#rightsOrigin.action", wrapper);
    var $journal = $("#journal.action", wrapper);

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
        html: '<a>'+(new veda.IndividualModel('v-s:Question')['rdfs:label'].join(" "))+'</a>'
      }));
      stask.append($('<li/>', {
        style:'cursor:pointer',
        click: function() {veda.Util.send(individual, template, 'v-wf:instructionRouteStartForm', true)},
        html: '<a>'+(new veda.IndividualModel('v-s:Instruction')['rdfs:label'].join(" "))+'</a>'
      }));
      stask.append($('<li/>', {
        style:'cursor:pointer',
        click: function() {veda.Util.send(individual, template, 'v-wf:taskRouteStartForm', true)},
        html: '<a>'+(new veda.IndividualModel('v-s:Introduction')['rdfs:label'].join(" "))+'</a>'
      }));
      stask.append($('<li/>', {
        style:'cursor:pointer',
        click: function() {veda.Util.send(individual, template, 'v-wf:distributionRouteStartForm', true)},
        html: '<a>'+(new veda.IndividualModel('v-s:Distribution')['rdfs:label'].join(" "))+'</a>'
      }));
      stask.append($('<li/>', {
        style:'cursor:pointer',
        click: function() {veda.Util.send(individual, template, 'v-wf:coordinationRouteStartForm', true)},
        html: '<a>'+(new veda.IndividualModel('v-s:Coordination')['rdfs:label'].join(" "))+'</a>'
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
          spec = specs[property_uri] ? new veda.IndividualModel( specs[property_uri] ) : undefined;
      if (property_uri === "@") {
        propertyContainer.text(individual.id);
        return;
      }
      renderPropertyValues(individual, property_uri, propertyContainer, props_ctrls, template, mode);

      // Re-render all property values if model's property was changed
      function propertyModifiedHandler() {
        renderPropertyValues(individual, property_uri, propertyContainer, props_ctrls, template, mode);
      }
      individual.on(property_uri, propertyModifiedHandler);
      template.one("remove", function () {
        individual.off(property_uri, propertyModifiedHandler);
      });
    });

    // Fetch related individuals all together
    var prefetch_args = [1];

    // Related resources
    var rels = $("[rel]:not(veda-control):not([rel] *):not([about] *)", wrapper);
    rels.map( function () {
      var rel_uri = $(this).attr("rel");
      if ( individual.hasValue(rel_uri) ) {
        prefetch_args.push(rel_uri);
      }
    });
    if (prefetch_args.length > 1) {
      individual.prefetch.apply(individual, prefetch_args);
    }

    // Fetch about resources alltogether
    var abouts = [];
    $("[about]:not([rel] *):not([about] *)", wrapper).map( function () {
      var about_uri = $(this).attr("about");
      if (about_uri !== "@" && !veda.cache[about_uri] ) {
        abouts.push(about_uri);
      }
    });
    if (abouts.length) {
      get_individuals(veda.ticket, abouts).map(function (item) {
        var about = new veda.IndividualModel(item);
      });
    }

    // Related resources & about resources
    rels.map( function () {
      //$("[rel]:not(veda-control):not([rel] *):not([about] *)", wrapper).map( function () {
      var relContainer = $(this),
          about = relContainer.attr("about"),
          rel_uri = relContainer.attr("rel"),
          isEmbedded = relContainer.attr("data-embedded") === "true",
          spec = specs[rel_uri] ? new veda.IndividualModel( specs[rel_uri] ) : undefined,
          rel_inline_template = relContainer.html().trim(),
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
        relTemplate = templateIndividual["v-ui:template"][0].toString();
      } else if ( rel_inline_template.length ) {
        relTemplate = rel_inline_template;
      }

      template.on("view edit search", function (e) {
        if (e.type === "view") {
          relContainer.sortable("disable");
        } else if (e.type === "edit") {
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
        } else if (e.type === "search") {
          relContainer.sortable("enable");
        }
        e.stopPropagation();
      });

      var values = about[rel_uri], rendered = {}, counter = 0;

      relContainer.empty();

      propertyModifiedHandler(values);
      about.on(rel_uri, propertyModifiedHandler);
      template.one("remove", function () {
        about.off(rel_uri, propertyModifiedHandler);
      });

      if (isEmbedded) {
        embeddedHandler(values);
        about.on(rel_uri, embeddedHandler);
        template.one("remove", function () {
          about.off(rel_uri, embeddedHandler);
        });
      }

      // Re-render link property if its' values were changed
      function propertyModifiedHandler (values) {
        ++counter;
        try {
          if (values.length) {
            values.map(function (value) {
              if (value.id in rendered) {
                rendered[value.id].cnt = counter;
                return;
              }
              value.load().then(function (value) {
                var renderedTmpl = renderRelationValue (about, rel_uri, value, relContainer, relTemplate, isEmbedded, embedded, isAbout, template, mode);
                rendered[value.id] = {tmpl: renderedTmpl, cnt: counter};
              })
            });
          } else {
            relContainer.empty();
          }
        } catch (error) {
          if (error instanceof TypeError) {
            var notify = veda.Notify ? new veda.Notify() : function () {};
            notify("warning", {name: "Error", message: "Attribute undefined: " + rel_uri});
          }
        }
        // Remove rendered templates for removed values
        for (var i in rendered) {
          if (rendered[i].cnt === counter) continue;
          rendered[i].tmpl.remove();
          delete rendered[i];
        }
      }

      function embeddedHandler(values) {
        if (mode === "edit") {
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
      propertyModifiedHandler();

      function propertyModifiedHandler() {
        if (about[property_uri] !== undefined) {
          var formatted = about[property_uri].map(veda.Util.formatValue).join(" ");
          propertyContainer.text( formatted );
        }
      }
      about.on(property_uri, propertyModifiedHandler);
      template.one("remove", function () {
        about.off(property_uri, propertyModifiedHandler);
      });

      var updateService = new veda.UpdateService();
      updateService.subscribe(about.id);
      template.one("remove", function () {
        updateService.unsubscribe(about.id);
      });
    });

    // Validation with support of embedded templates (arbitrary depth)

    // Initial validation state
    var validation = {state: true};
    template.data("validation", validation);

    function validateTemplate (e) {
      if ( Object.keys(validation).length === 0) {
        if ( !template.data("isEmbedded") ) {
          e.stopPropagation();
        }
        return;
      }
      if (mode === "edit") {
        Object.keys(validation).map( function (property_uri) {
          if (property_uri === "state") { return; }
          var spec = specs[property_uri] ? new veda.IndividualModel( specs[property_uri] ) : undefined;
          validation[property_uri] = validate(individual, property_uri, spec);
        });
        template.trigger("validate");
        validation.state = Object.keys(validation).reduce( function (acc, property_uri) {
          if (property_uri === "state") { return acc; }
          return acc && validation[property_uri].state;
        }, true);
        validation.state = validation.state && embedded.reduce(function (acc, template) {
              var embeddedValidation = template.data("validation");
              return embeddedValidation ? acc && embeddedValidation.state : acc;
            }, true);
        template.trigger("internal-validated");
      }
      // "validate" event should bubble up to be handled by parent template only if current template is embedded
      if ( !template.data("isEmbedded") ) {
        e.stopPropagation();
      }
    }
    template.on("internal-validate", validateTemplate);

    function triggerValidation() {
      if (mode === "edit") {
        template.trigger("internal-validate");
      }
    };
    individual.on("propertyModified", triggerValidation);
    template.one("remove", function () {
      individual.off("propertyModified", triggerValidation);
    });
    template.on("edit", triggerValidation);

    // Handle validation events from template
    template.on("validate", function (e) {
      e.stopPropagation();
    });
    template.on("validated", function (e, validationResult) {
      e.stopPropagation();
      if (mode === "edit") {
        // Merge template validation results with internal validation results
        Object.keys(validationResult).map(function (property_uri) {
          if (property_uri === "state") { return; }
          validation[property_uri] = validationResult[property_uri];
        });
        validation.state = validation.state && validationResult.state;
      }
    });

    template.on("internal-validated", function (e) {
      if (validation.state) {
        $save.removeAttr("disabled");
        $send.removeAttr("disabled");
        $sendButtons.removeAttr("disabled");
        $createReport.removeAttr("disabled");
        $createReportButtons.removeAttr("disabled");
      } else {
        $save.attr("disabled", "disabled");
        $send.attr("disabled", "disabled");
        $sendButtons.attr("disabled", "disabled");
        $createReport.attr("disabled", "disabled");
        $createReportButtons.attr("disabled", "disabled");
      }
      e.stopPropagation();
    });

    // Property control
    $("veda-control[property]:not([rel] *):not([about] *)", wrapper).map( function () {

      var control = $(this),
          property_uri = control.attr("property"),
          property = new veda.IndividualModel(property_uri),
          type = control.attr("data-type") || property["rdfs:range"][0].id,
          spec = specs[property_uri] ? new veda.IndividualModel( specs[property_uri] ) : undefined,
          controlType = control.attr("data-type") ? $.fn["veda_" + control.attr("data-type")] : $.fn.veda_generic;

      // Initial validation state
      validation[property_uri] = {state: true, cause: []};

      function validatedHandler(e) {
        if ( validation.state || !validation[property_uri] || validation[property_uri].state === true ) {
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
      template.on("internal-validated", validatedHandler);

      template.on("view edit search", function (e) {
        e.stopPropagation();
        control.trigger(e.type);
      });

      function assignDefaultValue (e) {
        if ( spec && spec.hasValue("v-ui:defaultValue") && !individual.hasValue(property_uri) ) {
          individual[property_uri] = spec["v-ui:defaultValue"];
        }
        e.stopPropagation();
      }
      template.on("edit", assignDefaultValue);

      var opts = {
        individual: individual,
        property_uri: property_uri,
        spec: spec,
        mode: mode
      };

      controlType.call(control, opts);

      props_ctrls[property_uri] ? props_ctrls[property_uri].push(control) : props_ctrls[property_uri] = [ control ];

    });

    // Relation control
    $("veda-control[rel]:not([rel] *):not([about] *)", wrapper).map( function () {

      var control = $(this),
          rel_uri = control.attr("rel"),
          spec = specs[rel_uri] ? new veda.IndividualModel( specs[rel_uri] ) : undefined,
          rel = new veda.IndividualModel(rel_uri),
          controlType = control.attr("data-type") ? $.fn["veda_" + control.attr("data-type")] : $.fn.veda_link;

      // Initial validation state
      validation[rel_uri] = {state: true, cause: []};

      function validatedHandler(e) {
        if ( validation.state || !validation[rel_uri] || validation[rel_uri].state === true) {
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
      template.on("internal-validated", validatedHandler);

      template.on("view edit search", function (e) {
        e.stopPropagation();
        control.trigger(e.type);
      });

      function assignDefaultValue (e) {
        if ( spec && spec.hasValue("v-ui:defaultValue") && !individual.hasValue(rel_uri) ) {
          individual[rel_uri] = spec["v-ui:defaultValue"];
        }
        e.stopPropagation();
      }
      template.on("edit", assignDefaultValue);

      var opts = {
        individual: individual,
        rel_uri: rel_uri,
        spec: spec,
        mode: mode
      };
      controlType.call(control, opts);

    });

    return template;
  }

  function renderPropertyValues(individual, property_uri, propertyContainer, props_ctrls, template, mode) {
    propertyContainer.empty();
    individual[property_uri].map( function (value, i) {
      var valueHolder = $("<span class='value-holder'></span>");
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
        valTemplate = $(relTemplate);
        value.present(relContainer, valTemplate, mode);
      } else {
        value.present(relContainer, undefined, mode);
      }
      valTemplate = $("[resource='" + value.id + "']", relContainer).first();
      valTemplate.data("isEmbedded", true);
      embedded.push(valTemplate);
      valTemplate.one("remove", function () {
        if (embedded.length) {
          var index = embedded.indexOf(valTemplate);
          if ( index >= 0 ) embedded.splice(index, 1);
        }
      });
    } else {
      if (relTemplate) {
        valTemplate = $(relTemplate);
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
        if ( value.is("v-s:Embedded") && value.hasValue("v-s:parent", individual) ) {
          value.delete();
        }
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
          );
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
        case "v-ui:DatatypePropertySpecification" :
          if (spec.hasValue("v-ui:minValue")) {
            var minValueState = (value >= spec["v-ui:minValue"][0]);
            result.state = result.state && minValueState;
            if (!minValueState) {
              result.cause.push("v-ui:minValue");
            }
          }
          if (spec.hasValue("v-ui:maxValue")) {
            var maxValueState = (value <= spec["v-ui:maxValue"][0]);
            result.state = result.state && maxValueState;
            if (!maxValueState) {
              result.cause.push("v-ui:maxValue");
            }
          }
          if (spec.hasValue("v-ui:minLength")) {
            var minLengthState = (value.toString().length >= spec["v-ui:minLength"][0]);
            result.state = result.state && minLengthState;
            if (!minLengthState) {
              result.cause.push("v-ui:minLength");
            }
          }
          if (spec.hasValue("v-ui:maxLength")) {
            var maxLengthState = (value.toString().length <= spec["v-ui:maxLength"][0]);
            result.state = result.state && maxLengthState;
            if (!maxLengthState) {
              result.cause.push("v-ui:maxLength");
            }
          }
          break;
        case "v-ui:ObjectPropertySpecification" :
          break;
      }
      return result;
    }, result);
    return result;
  }

});
