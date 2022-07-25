def Settings( **kwargs ):
  if kwargs[ 'language' ] == 'rust':
      settings = { 'ls': {  } }
      settings['ls']['cargo'] = { 'features': [], 'tests': True }
      settings['ls']['workspace'] = { 'symbol': { 'search': { 'limit': 2048 } } }
      return settings
