def Settings( **kwargs ):
  if kwargs[ 'language' ] == 'rust':
      settings = { 'ls': {  } }
      settings['ls']['cargo'] = { 'features': ['unit_test', 'integration_test', 'aws_test'], 'tests': True }
      settings['ls']['workspace'] = { 'symbol': { 'search': { 'limit': 4096 } } }
      return settings
