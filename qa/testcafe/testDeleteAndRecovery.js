import Basic from './basic';
import config from './config';
import { Selector, t } from 'testcafe';
  fixture `test Delete And Recovery`
    .page `${config.baseUrl}`;
  const basic = new Basic();
  const timeStamp = '' + Math.round(+new Date()/1000);
  const queryStartForm = "'rdfs:label' == '" + timeStamp + "'" ;
  const queryDeletedStartForm = "'rdfs:label' == '" + timeStamp + "' && 'v-s:deleted' == 'true'" ;
  const menu = Selector('#menu');
  const create = Selector('li[id="menu"] li[resource="v-s:Create"]');
  const searchTargetTypeInput = Selector('veda-control[rel="v-fc:targetType"] .form-control');
  const startFormSuggestion = Selector('.suggestion[resource="v-wf:StartForm"]');
  const labelInput = Selector('veda-control[property="rdfs:label"] .form-control');
  const saveBtn = Selector('#save');
  const fulltextSearch = Selector('li[about="v-fs:FulltextSearch"]');
  const fulltextQueryInput = Selector('veda-control[property="*"] .form-control');
  const searchBtn = Selector('.search-button');
  const searchResult = Selector('.results a.glyphicon.glyphicon-search');
  const deleteBtn = Selector('#delete');
  const recoverBtn = Selector('.recover');
  const advancedSearch = Selector('.advanced-toggle');
  const deleteTypeValue = Selector('div[rel="rdf:type"] .rel-actions .button-delete');
  const estimatedResults = Selector('.stats-top span[property="v-fs:estimated"]');

  test('testDeleteAndRecovery', async t => {
    basic.login('karpovrt', '123');
    await t
      .setNativeDialogHandler(() => true)
      .click(menu)
      .click(create)
      .typeText(searchTargetTypeInput, 'Стартовая форма')
      .click(startFormSuggestion)
      .typeText(labelInput, timeStamp)
      .click(saveBtn)
      .click(fulltextSearch)
      .typeText(fulltextQueryInput, queryStartForm)
      .click(searchBtn)
      .click(searchResult)
      .click(deleteBtn)
      .click(fulltextSearch)
      .click(fulltextQueryInput)
      .pressKey('ctrl+a delete')
      .typeText(fulltextQueryInput, queryDeletedStartForm)
      .click(searchBtn)
      .click(searchResult)
      .click(recoverBtn)
      .click(fulltextSearch)
      .click(advancedSearch)
      .click(deleteTypeValue)
      .click(fulltextQueryInput)
      .pressKey('ctrl+a delete')
      .typeText(fulltextQueryInput, timeStamp)
      .click(searchBtn)
      .expect(estimatedResults.innerText).eql('1');
  });
